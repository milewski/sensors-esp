use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::{FromValueType, Peripherals};
use crate::lcd::Direction;

mod lcd;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For LCD
    let sda = peripherals.pins.gpio2;
    let scl = peripherals.pins.gpio1;

    let mut display = lcd::LCD::new(peripherals.i2c1, sda, scl, FreeRtos::delay_ms)?;

    display.initialize()?;
    display.write_str("Hello")?;
    display.cursor_move_to(1, 0)?;
    display.write_str("World")?;

    let mut scroll_direction = lcd::Direction::Right;
    let mut counter = 0;

    loop {
        match scroll_direction {
            Direction::Right => {
                counter += 1;
            }
            Direction::Left => {
                counter -= 1;
            }
        };

        FreeRtos::delay_ms(400);
        display.scroll(scroll_direction)?;

        scroll_direction = match counter {
            value if value >= 11 => Direction::Left,
            value if value <= 0 => Direction::Right,
            _ => scroll_direction
        };
    }
}

