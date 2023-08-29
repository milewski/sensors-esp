use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{AnyIOPin, PinDriver};
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::spi::{SpiConfig, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::spi::config::DriverConfig;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let sck = peripherals.pins.gpio6;
    let cs = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio4;
    let driver_config = DriverConfig::default();

    let driver = SpiDriver::new(peripherals.spi2, sck, mosi, Option::<AnyIOPin>::None, &driver_config)?;
    let spi_config = SpiConfig::default();
    let mut spi = SpiDeviceDriver::new(driver, Option::<AnyIOPin>::None, &spi_config)?;

    let mut cs = PinDriver::output(cs)?;

    // Power Up Device
    cs.set_low()?;
    spi.write(&[0x0C, 0x01])?;
    cs.set_high()?;

    // Set up Decode Mode
    cs.set_low()?;
    spi.write(&[0x09, 0x00])?;
    cs.set_high()?;

    // Configure Scan Limit
    cs.set_low()?;
    spi.write(&[0x0B, 0x07])?;
    cs.set_high()?;

    // Configure Scan Limit
    cs.set_low()?;
    spi.write(&[0x0A, 0x00])?;
    cs.set_high()?;

    for i in 1..9 {
        cs.set_low()?;
        spi.write(&[i, 0])?;
        cs.set_high()?;
    }

    let mut matrix = [0u8; 128];

    matrix[0] = 1;
    matrix[1] = 1;
    matrix[2] = 1;
    matrix[3] = 1;
    matrix[4] = 1;
    matrix[5] = 1;
    matrix[6] = 1;
    matrix[7] = 1;

    matrix[120] = 1;
    matrix[121] = 1;
    matrix[122] = 1;
    matrix[123] = 1;
    matrix[124] = 1;
    matrix[125] = 1;
    matrix[126] = 1;
    matrix[127] = 1;

    // Rotate Matrix

    loop {
        FreeRtos::delay_ms(100);

        for i in 1..=8 {
            cs.set_low()?;
            spi.write(&[i, 0])?;
            spi.write(&[i, 0])?;
            cs.set_high()?;
        }

        for row in 0..8 {
            let mut display_top: u8 = 0;
            let mut display_bottom: u8 = 0;

            for column in 0..8 {
                let index = column * 8 + row;

                display_top |= (matrix[0 + index] << (7 - column));
                display_bottom |= (matrix[64 + index] << (7 - column));
            }

            cs.set_low()?;
            spi.write(&[row as u8 + 1, display_top])?;
            spi.write(&[row as u8 + 1, display_bottom])?;
            cs.set_high()?;
        }

        // for (index, row) in matrix.chunks(8).enumerate() {
        //
        // }

        FreeRtos::delay_ms(100);
    }
}
