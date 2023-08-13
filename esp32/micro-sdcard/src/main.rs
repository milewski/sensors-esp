use std::fmt::{Display, Formatter};
use std::sync::Mutex;

use anyhow::anyhow;
use embedded_graphics::Drawable;
use embedded_graphics::prelude::*;
use embedded_sdmmc::{BlockDevice, TimeSource};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{InputPin, OutputPin};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::spi::SpiAnyPins;
use esp_idf_hal::units::FromValueType;
use rotary_encoder_embedded::Direction;
use ssd1306::prelude::DisplayConfig;

use crate::display::TinyDisplay;
use crate::encoder::RotaryEncoder;
use crate::file_list::FileList;
use crate::micro_sdcard::MicroSdCard;

mod display;
mod file_list;
mod micro_sdcard;
mod encoder;

#[derive(Debug)]
enum CustomError {
    UnableToTakePeripherals,
    UnableToInitializeCSPin,
    UnableToGetVolume,
    UnableToOpenDirectory,
    FailedToInitializeI2cDriver,
    FailedToInitializeDisplay,
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

    // For Micro SD Card
    let cs = peripherals.pins.gpio10;
    let sck = peripherals.pins.gpio3;
    let mosi = peripherals.pins.gpio2;
    let miso = peripherals.pins.gpio1;

    // For Display
    let scl = peripherals.pins.gpio11;
    let sda = peripherals.pins.gpio12;

    // For Rotary Encoder
    let s1_pin = peripherals.pins.gpio16;
    let s2_pin = peripherals.pins.gpio21;
    let key_pin = peripherals.pins.gpio17;

    let display = TinyDisplay::new(peripherals.i2c0, sda, scl)?;
    let mut sdcard = MicroSdCard::new(peripherals.spi2, sck, mosi, miso, cs)?;
    let mut encoder = RotaryEncoder::new(s1_pin, s2_pin, key_pin)?;

    let files = sdcard.list_files()?;

    let mut file_list = FileList::new(display, files)?;
    file_list.draw()?;

    let file_list = Mutex::new(file_list);

    encoder.handle(Box::new(move |direction, is_clicked| {
        let mut locked = file_list
            .lock()
            .map_err(|error| anyhow!("unable to acquire lock: {:?}", error))?;

        match direction {
            Direction::Clockwise => locked.scroll_up(),
            Direction::Anticlockwise => locked.scroll_down(),
            Direction::None => Ok(())
        }
    }));

    loop {
        encoder.update()?;
        FreeRtos::delay_ms(1);
    }
}
