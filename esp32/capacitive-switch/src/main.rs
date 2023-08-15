use std::sync::Mutex;
use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;
use shared::tiny_display::TinyDisplay;
use crate::capacitive_sensor::CapacitiveSensor;
use crate::game::Game;

mod capacitive_sensor;
mod game;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let scl = peripherals.pins.gpio12;
    let sda = peripherals.pins.gpio11;

    // For Green LED
    let led = peripherals.pins.gpio18;

    // For Capacitive Sensor
    let one = peripherals.pins.gpio1;
    let two = peripherals.pins.gpio2;
    let three = peripherals.pins.gpio3;
    let four = peripherals.pins.gpio10;

    let led = PinDriver::output(led)?;
    let display = TinyDisplay::new(peripherals.i2c0, sda, scl)?;
    let mut sensor = CapacitiveSensor::new(one, two, three, four)?;

    let game = Mutex::new(Game::new(display, led)?);

    sensor.on_touch(Box::new(move |button| {
        game.lock()
            .map_err(|_| anyhow!("failed to acquire lock"))?
            .update(button.to_string())
    }));

    loop {
        sensor.update()?;
        FreeRtos::delay_ms(1);
    }
}
