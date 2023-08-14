use std::sync::Mutex;

use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{InputPin, PinDriver};
use esp_idf_hal::prelude::*;
use profont::{PROFONT_12_POINT, PROFONT_24_POINT};

use shared::tiny_display::TinyDisplay;

use crate::capacitive_sensor::CapacitiveSensor;

mod capacitive_sensor;

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

    let mut led = Mutex::new(PinDriver::output(led)?);
    let mut sensor = CapacitiveSensor::new(one, two, three, four)?;
    let mut display = Mutex::new(TinyDisplay::new(peripherals.i2c1, sda, scl)?);

    let mut secret = Mutex::new(["_".to_string(), "__".to_string(), "_".to_string(), "_".to_string()]);
    let mut index = Mutex::new(0);
    let secret_code = [fastrand::u8(1..=4), fastrand::u8(1..=4), fastrand::u8(1..=4), fastrand::u8(1..=4)];

    println!("The secret code is: {}", secret_code.iter().fold(String::new(), |a, b| format!("{}{}", a, b)));

    {
        let mut display = display.lock().map_err(|_| anyhow!("failed to lock display"))?;
        display.clear();
        display.draw_text(&"Enter Code".to_string(), PROFONT_12_POINT, 25, 20)?;

        draw_secret(&mut display, &secret.lock().unwrap())?;
    }

    sensor.on_touch(Box::new(move |button| {
        let mut index = index.lock().map_err(|_| anyhow!("failed to lock index"))?;
        let mut secret = secret.lock().map_err(|_| anyhow!("failed to lock secret"))?;
        let mut display = display.lock().map_err(|_| anyhow!("failed to lock display"))?;
        let mut led = led.lock().map_err(|_| anyhow!("failed to lock led"))?;

        secret[*index] = button.to_string();

        *index += 1;

        draw_secret(&mut display, &secret)?;

        if *index == 4 {
            let flat_secret: [u8; 4] = secret
                .iter()
                .map(|value| value.parse().unwrap())
                .collect::<Vec<u8>>()
                .try_into()
                .map_err(|_| anyhow!("failed to convert secret"))?;

            display.clear();

            if flat_secret == secret_code {
                display.draw_text(&"Congratulations".to_string(), PROFONT_12_POINT, 5, 35)?;
                led.set_high()?;
            } else {
                FreeRtos::delay_ms(200);

                display.draw_text(&"Try Again!".to_string(), PROFONT_12_POINT, 25, 35)?;
                display.flush()?;

                FreeRtos::delay_ms(500);

                // Reset the game
                *secret = ["_".to_string(), "__".to_string(), "_".to_string(), "_".to_string()];
                *index = 0;

                display.clear();
                display.draw_text(&"Enter Code".to_string(), PROFONT_12_POINT, 25, 20)?;

                draw_secret(&mut display, &secret)?;
            }

            display.flush()?;
        }

        Ok(())
    }));

    loop {
        sensor.update()?;
        FreeRtos::delay_ms(1);
    }
}

fn draw_secret(display: &mut TinyDisplay, secret: &[String; 4]) -> anyhow::Result<()> {
    for (index, code) in secret.iter().enumerate() {
        display.draw_text(&code.to_string(), PROFONT_24_POINT, 25 + (20 * index as i32), 50)?;
    }

    display.flush()
}