use std::fmt::Display;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Level, PinDriver};
use esp_idf_hal::peripherals::Peripherals;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let mut sensor = PinDriver::input(peripherals.pins.gpio2)?;
    let mut red_led = PinDriver::output(peripherals.pins.gpio3)?;
    let mut green_led = PinDriver::output(peripherals.pins.gpio10)?;
    let mut active_buzzer = PinDriver::output(peripherals.pins.gpio11)?;

    loop {
        match sensor.get_level() {
            Level::Low => {
                red_led.set_high()?;
                green_led.set_low()?;
                active_buzzer.set_high()?;
            }
            Level::High => {
                green_led.set_high()?;
                red_led.set_low()?;
                active_buzzer.set_low()?;
            }
        }

        FreeRtos::delay_ms(1);
    }
}
