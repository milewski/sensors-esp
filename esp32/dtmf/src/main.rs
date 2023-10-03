use std::sync::Mutex;
use anyhow::anyhow;
use esp_idf_hal::gpio::InputPin;
use esp_idf_hal::prelude::Peripherals;
use profont::PROFONT_24_POINT;

use shared::tiny_display::TinyDisplay;
use crate::dtmf::DTMF;

mod dtmf;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For DTMF
    let q1 = peripherals.pins.gpio1;
    let q2 = peripherals.pins.gpio2;
    let q3 = peripherals.pins.gpio42;
    let q4 = peripherals.pins.gpio41;
    let st = peripherals.pins.gpio4;

    // For Display
    let scl = peripherals.pins.gpio7;
    let sda = peripherals.pins.gpio6;

    let mut instance = DTMF::new(q1, q2, q3, q4, st)?;

    let mut display = TinyDisplay::new(peripherals.i2c0, sda, scl)?;
    display.clear();

    let display = Mutex::new(display);

    instance.on_pressed(Box::new(move |number| {
        let mut display = display.lock().expect("failed to acquire lock");

        display.clear();
        display.draw_text(&number.to_string(), PROFONT_24_POINT, 55, 42).expect("failed to draw text");
        display.flush().expect("failed to flush display");
    }));

    instance.listen();

    Ok(())
}

