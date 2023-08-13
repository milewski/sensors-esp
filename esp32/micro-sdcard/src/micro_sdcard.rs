use anyhow::anyhow;
use embedded_sdmmc::{Directory, DirEntry, File, SdCard, TimeSource, Timestamp, Volume, VolumeIdx, VolumeManager};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio10, InputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::spi::{Dma, SpiAnyPins, SpiConfig, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::spi::config::DriverConfig;

pub struct SdLocalTimeSource;

impl TimeSource for SdLocalTimeSource {
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

pub struct MicroSdCard<'d, CS: OutputPin> {
    manager: VolumeManager<SdCard<SpiDeviceDriver<'d, SpiDriver<'d>>, PinDriver<'d, CS, Output>, FreeRtos>, SdLocalTimeSource>,
    volume: Volume,
    root: Directory,
}

impl<'d, CS: OutputPin> MicroSdCard<'d, CS> {
    pub fn new(
        spi2: impl Peripheral<P=impl SpiAnyPins> + 'd,
        sck: impl Peripheral<P=impl OutputPin> + 'd,
        mosi: impl Peripheral<P=impl OutputPin> + 'd,
        miso: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        cs: CS,
    ) -> anyhow::Result<MicroSdCard<'d, CS>> {
        let driver_config = DriverConfig::default().dma(Dma::Disabled);
        let driver = SpiDriver::new(spi2, sck, mosi, Some(miso), &driver_config)?;

        let spi_config = SpiConfig::default();
        let device = SpiDeviceDriver::new(driver, Option::<Gpio10>::None, &spi_config)?;
        let cs_pin = PinDriver::output(cs)?;
        let sdcard = SdCard::new(device, cs_pin, FreeRtos);

        let mut manager = VolumeManager::new(sdcard, SdLocalTimeSource);

        let mut volume = manager
            .get_volume(VolumeIdx(0))
            .map_err(|error| anyhow!("failed to get volume 0: {:?}", error))?;

        let root = manager
            .open_root_dir(&volume)
            .map_err(|error| anyhow!("failed to open root directory: {:?}", error))?;

        Ok(Self { manager, volume, root })
    }

    pub fn read_file(&mut self, file: &mut File) -> anyhow::Result<Vec<u8>> {
        let mut content = vec![];

        // While the end of the file is not reach, create chunks of 32 bytes and fill it in with the incoming data
        while !file.eof() {
            let mut buffer = [0u8; 32];
            let num_read = self.manager.read(&self.volume, file, &mut buffer)
                .map_err(|error| anyhow!("failed to read file content: {:?}", error))?;

            content.extend_from_slice(&buffer[0..num_read]);
        }

        Ok(content)
    }

    pub fn list_files(&mut self) -> anyhow::Result<Vec<DirEntry>> {
        let mut files = vec![];

        self.manager
            .iterate_dir(&self.volume, &self.root, |entry| files.push(entry.clone()))
            .map_err(|error| anyhow!("failed to list root directory: {:?}", error))?;

        Ok(files)
    }
}