use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::AnyIOPin;
use esp_idf_hal::prelude::Peripherals;
use profont::{PROFONT_14_POINT};
use shared::tiny_display::TinyDisplay;
use crate::accelerometer::Accelerometer;

mod accelerometer;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // For Accelerometer
    let sda = peripherals.pins.gpio44;
    let scl = peripherals.pins.gpio43;

    // For Display
    let display_sda = peripherals.pins.gpio2;
    let display_scl = peripherals.pins.gpio1;

    let mut display = TinyDisplay::new(peripherals.i2c0, display_sda, display_scl)?;
    let mut accelerometer = Accelerometer::new(peripherals.i2c1, sda, scl, Option::<AnyIOPin>::None)?;

    accelerometer.start()?;

    println!("Device ID: 0x{:0X?}", accelerometer.device_id()?);

    loop {
        let (x, y, z) = accelerometer.acceleration()?;

        display.clear();
        display.draw_text(&format!("X:{:>4}", x), PROFONT_14_POINT, 30, 15)?;
        display.draw_text(&format!("Y:{:>4}", y), PROFONT_14_POINT, 30, 35)?;
        display.draw_text(&format!("Z:{:>4}", z), PROFONT_14_POINT, 30, 55)?;
        display.flush()?;

        FreeRtos::delay_ms(50);
    }
}

