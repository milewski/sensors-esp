use std::fmt::{Display, Formatter};
use std::time::Duration;

use button_driver::{Button, ButtonConfig};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Gpio1, Gpio43, Input, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use rotary_encoder_embedded::{Direction, RotaryEncoder};
use rotary_encoder_embedded::standard::StandardMode;

// Hex digits from 0 to F
const DIGITS: [u8; 16] = [
    0x3f, 0x06, 0x5b, 0x4f,
    0x66, 0x6d, 0x7d, 0x07,
    0x7f, 0x6f, 0x77, 0x7c,
    0x39, 0x5e, 0x79, 0x71,
];

const MAX_BRIGHTNESS: u8 = 7;
const MAX_COUNTER: u16 = 9999;

#[derive(Debug)]
enum CustomError {
    UnableToTakePeripherals,
    UnableToSetBrightness,
    UnableToPrint,
    FailedToInitializeDisplay,
    FailedToClearDisplay,
}

impl std::error::Error for CustomError {}

impl Display for CustomError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error: {:?}", self)
    }
}

#[derive(PartialEq, Debug)]
enum Mode {
    Counter,
    SetBrightness,
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let mut counter: u16 = 0;
    let mut brightness: u8 = 5;
    let mut current_mode;

    let peripherals = Peripherals::take().ok_or(CustomError::UnableToTakePeripherals)?;

    // For rotary encoder
    let s1_pin = peripherals.pins.gpio1;
    let s2_pin = peripherals.pins.gpio43;
    let key_pin = peripherals.pins.gpio3;

    // For display
    let dio_pin = peripherals.pins.gpio44;
    let clock_pin = peripherals.pins.gpio18;

    let rotary_dt = PinDriver::input(s1_pin)?;
    let rotary_clk = PinDriver::input(s2_pin)?;
    let button = PinDriver::input(key_pin)?;

    let mut button = Button::new(button, ButtonConfig {
        debounce: Duration::from_micros(300),
        release: Duration::from_millis(200),
        hold: Duration::from_millis(100),
        mode: button_driver::Mode::default(),
    });

    let mut encoder = RotaryEncoder::new(rotary_dt, rotary_clk).into_standard_mode();

    // Setup display
    let mut dio = PinDriver::input_output(dio_pin)?;
    let mut clock = PinDriver::input_output(clock_pin)?;

    let mut delay = FreeRtos;
    let mut display = tm1637::TM1637::new(&mut clock, &mut dio, &mut delay);

    display.init().map_err(|_| CustomError::FailedToInitializeDisplay)?;
    display.clear().map_err(|_| CustomError::FailedToClearDisplay)?;
    display.set_brightness(brightness).map_err(|_| CustomError::UnableToSetBrightness)?;

    loop {
        button.tick();
        encoder.update();

        // Reset counter when button is clicked
        if button.is_clicked() {
            counter = 0
        }

        // Switch mode when button is held
        current_mode = match button.raw_state().is_held() {
            true => Mode::SetBrightness,
            false => Mode::Counter
        };

        handle_encoder(&encoder, &current_mode, &mut counter, &mut brightness);

        if current_mode == Mode::SetBrightness {
            display.set_brightness(brightness).map_err(|_| CustomError::UnableToSetBrightness)?;
            display.print_raw(0, &get_digits(brightness).as_slice()).map_err(|_| CustomError::UnableToPrint)?;
        }

        if current_mode == Mode::Counter {
            display.print_raw(0, &get_digits(counter).as_slice())
                .map_err(|_| CustomError::UnableToPrint)?;
        }

        button.reset();

        FreeRtos::delay_ms(1);
    }
}

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

fn handle_encoder(
    encoder: &RotaryEncoder<StandardMode, PinDriver<Gpio1, Input>, PinDriver<Gpio43, Input>>,
    current_mode: &Mode,
    counter: &mut u16,
    brightness: &mut u8,
) {
    match encoder.direction() {
        Direction::Clockwise => {
            match current_mode {
                Mode::Counter => {
                    if *counter > 0 {
                        *counter -= 1;
                    }
                }
                Mode::SetBrightness => {
                    if *brightness > 0 {
                        *brightness -= 1;
                    }
                }
            }
        }
        Direction::Anticlockwise => {
            match current_mode {
                Mode::Counter => {
                    if *counter < MAX_COUNTER {
                        *counter += 1;
                    }
                }
                Mode::SetBrightness => {
                    if *brightness < MAX_BRIGHTNESS {
                        *brightness += 1;
                    }
                }
            }
        }
        Direction::None => {
            // Do nothing
        }
    }
}