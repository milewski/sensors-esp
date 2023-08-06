use std::fmt::Display;

use button_driver::{Button, ButtonConfig};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::rmt::*;
use esp_idf_hal::rmt::config::{Loop, TransmitConfig};
use rotary_encoder_embedded::{Direction, RotaryEncoder};

use crate::player::Player;

mod songs;
pub mod song;
mod player;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // For rotary encoder
    let s1_pin = peripherals.pins.gpio21;
    let s2_pin = peripherals.pins.gpio10;
    let key_pin = peripherals.pins.gpio3;

    // For display
    let dio_pin = peripherals.pins.gpio16;
    let clock_pin = peripherals.pins.gpio13;

    // For buzzer
    let buzzer_pin = peripherals.pins.gpio2;

    let channel = peripherals.rmt.channel0;
    let config = TransmitConfig::new().looping(Loop::Endless);
    let transmitter: TxRmtDriver<'static> = TxRmtDriver::new(channel, buzzer_pin, &config)?;

    // Configure rotary encoder
    let rotary_dt = PinDriver::input(s1_pin)?;
    let rotary_clk = PinDriver::input(s2_pin)?;
    let key = PinDriver::input(key_pin)?;

    let mut button = Button::new(key, ButtonConfig::default());
    let mut encoder = RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode();

    // Setup display
    let mut dio = PinDriver::input_output(dio_pin)?;
    let mut clock = PinDriver::input_output(clock_pin)?;

    let mut delay = FreeRtos;
    let mut display = tm1637::TM1637::new(&mut clock, &mut dio, &mut delay);

    display.init().unwrap();
    display.clear().unwrap();
    display.set_brightness(5).unwrap();

    // Setup player
    let mut player = Player::new(transmitter);

    loop {
        button.tick();
        encoder.update();

        // When the button is pressed, stop the player and clear the display.
        if button.is_clicked() {
            player.stop()?;
            display.clear().unwrap();
        }

        match encoder.direction() {
            Direction::Clockwise => {
                player.previous()?;
                display.print_raw(0, &get_digits(player.current_track + 1).as_slice()).unwrap();
            }
            Direction::Anticlockwise => {
                player.next()?;
                display.print_raw(0, &get_digits(player.current_track + 1).as_slice()).unwrap();
            }
            Direction::None => {
                // Do nothing
            }
        }

        button.reset();

        FreeRtos::delay_ms(1);
    }
}

const DIGITS: [u8; 16] = [
    0x3f, 0x06, 0x5b, 0x4f,
    0x66, 0x6d, 0x7d, 0x07,
    0x7f, 0x6f, 0x77, 0x7c,
    0x39, 0x5e, 0x79, 0x71,
];

fn get_digits<Value>(value: Value) -> Vec<u8> where Value: Display {
    let mut numbers: Vec<u8> = value
        .to_string()
        .chars()
        .map(|d| d.to_digit(10).unwrap())
        .map(|d| DIGITS[(d & 0xf) as usize])
        .rev()
        .collect();

    numbers.resize(4, 0x00);
    numbers.reverse();

    numbers
}