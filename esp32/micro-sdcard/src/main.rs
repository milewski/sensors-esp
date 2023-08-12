use std::fmt::{Display, Formatter};

use embedded_sdmmc::{BlockDevice, File, Mode, SdCard, TimeSource, Timestamp, Volume, VolumeIdx, VolumeManager};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio10, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::{Dma, SpiConfig, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::spi::config::DriverConfig;

pub struct SdMmcClock;

impl TimeSource for SdMmcClock {
    fn get_timestamp(&self) -> Timestamp {
        Timestamp {
            year_since_1970: 0,
            zero_indexed_month: 0,
            zero_indexed_day: 0,
            hours: 0,
            minutes: 0,
            seconds: 0,
        }
    }
}

#[derive(Debug)]
enum CustomError {
    UnableToTakePeripherals,
    UnableToInitializeCSPin,
    UnableToGetVolume,
    UnableToOpenDirectory,
}

impl std::error::Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(CustomError::UnableToTakePeripherals)?;

    let cs = peripherals.pins.gpio10;
    let sck = peripherals.pins.gpio3;
    let mosi = peripherals.pins.gpio2;
    let miso = peripherals.pins.gpio1;

    let driver = SpiDriver::new(
        peripherals.spi2,
        sck,
        mosi,
        Some(miso),
        &DriverConfig::default().dma(Dma::Disabled),
    )?;

    let config = SpiConfig::default();
    let device = SpiDeviceDriver::new(driver, Option::<Gpio10>::None, &config)?;
    let cs_pin = PinDriver::output(cs)?;
    let sdcard = SdCard::new(device, cs_pin, FreeRtos);

    println!("Card size {:?}", sdcard.num_bytes());

    let mut volume_manager = VolumeManager::new(sdcard, SdMmcClock);
    let mut volume0 = volume_manager.get_volume(VolumeIdx(0)).unwrap();

    let root_dir = volume_manager.open_root_dir(&volume0).unwrap();

    let mut my_entry = None;

    volume_manager
        .iterate_dir(&volume0, &root_dir, |entry| {
            if entry.name.extension() == b"TXT" {
                my_entry = Some(entry.clone());
            }
        })
        .unwrap();

    if let Some(entry) = my_entry {
        let mut file = volume_manager
            .open_dir_entry(&mut volume0, entry, Mode::ReadOnly)
            .unwrap();

        let content = read_file(&mut volume_manager, volume0, &mut file);

        println!("{:?}", String::from_utf8(content))
    }

    loop {
        FreeRtos::delay_ms(100);
    }
}

fn read_file(volume_manager: &mut VolumeManager<impl BlockDevice, impl TimeSource>, volume: Volume, file: &mut File) -> Vec<u8> {
    let mut content = vec![];

    // While the end of the file is not reach, create chunks of 32 bytes and fill it in with the incoming data
    while !file.eof() {
        let mut buffer = [0u8; 32];
        let num_read = volume_manager.read(&volume, file, &mut buffer).unwrap();

        content.extend_from_slice(&buffer[0..num_read]);
    }

    content
}