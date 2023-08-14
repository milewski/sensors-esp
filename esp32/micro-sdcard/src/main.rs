use std::sync::Mutex;

use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::*;
use rotary_encoder_embedded::Direction;
use shared::tiny_display::TinyDisplay;

use crate::file_list::FileList;
use crate::micro_sdcard::MicroSdCard;
use crate::rotary_encoder::RotaryEncoder;

mod file_list;
mod micro_sdcard;
mod rotary_encoder;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

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
