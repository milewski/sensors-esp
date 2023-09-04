use anyhow::anyhow;
use esp_idf_hal::delay::{FreeRtos};
use esp_idf_hal::prelude::{FromValueType, Peripherals};

mod lcd;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let sda = peripherals.pins.gpio2;
    let scl = peripherals.pins.gpio1;

    // let mut driver = I2cDriver::new(peripherals.i2c1, sda, scl, &Config::new().baudrate(100.kHz().into()))?;

    let mut display = lcd::LCD::new(peripherals.i2c1, sda, scl)?;

    display.initialize();
    display.write_str("Works!");
    // set_cursor(&mut driver, &address, 1, 3);

    loop {
        FreeRtos::delay_ms(500);
    }
}

