use anyhow::anyhow;
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::{Point, Primitive, Size};
use embedded_graphics::primitives::{Circle, PrimitiveStyle, Rectangle};
use esp_idf_hal::delay::{BLOCK};
use esp_idf_hal::i2c::config::Config;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::prelude::{FromValueType, Hertz, Peripherals};
use shared::tiny_display::TinyDisplay;
use embedded_graphics::Drawable;

fn map_range(value: i32, from_range: (i32, i32), to_range: (i32, i32)) -> i32 {
    let from_min = from_range.0;
    let from_max = from_range.1;

    let to_min = to_range.0;
    let to_max = to_range.1;

    (value - from_min) * (to_max - to_min) / (from_max - from_min) + to_min
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For ADC/DAC Module
    let sda = peripherals.pins.gpio4;
    let scl = peripherals.pins.gpio5;

    // For display
    let display_sda = peripherals.pins.gpio2;
    let display_scl = peripherals.pins.gpio1;

    let driver_config = Config::default().baudrate(Hertz::from(100.kHz()));
    let mut driver = I2cDriver::new(peripherals.i2c1, sda, scl, &driver_config)?;

    let mut display = TinyDisplay::new(peripherals.i2c0, display_sda, display_scl)?;
    display.clear();

    let mut joystick_1_position = Point::new(0, 0);
    let mut joystick_2_position = Point::new(0, 0);

    display.flush()?;

    //                       A2 A1 A0
    let address: u8 = 0b1001_0__0__0;

    let joystick_x_1 = 0b0_000_0_001;
    let joystick_y_1 = 0b0_000_0_000;

    let joystick_x_2 = 0b0_000_0_011;
    let joystick_y_2 = 0b0_000_0_010;

    loop {
        let mut x = [0; 1];
        let mut y = [0; 1];

        let mut x_2 = [0; 1];
        let mut y_2 = [0; 1];

        driver.write_read(address, &[joystick_x_1], &mut y_2, BLOCK)?;
        driver.write_read(address, &[joystick_y_1], &mut x, BLOCK)?;

        driver.write_read(address, &[joystick_x_2], &mut y, BLOCK)?;
        driver.write_read(address, &[joystick_y_2], &mut x_2, BLOCK)?;

        let screen_width = 128 - 10;
        let screen_height = 64 - 10;

        joystick_1_position.x = map_range(x[0] as i32, (0, 255), (0, screen_width));
        joystick_1_position.y = map_range(y[0] as i32, (0, 255), (0, screen_height));

        joystick_2_position.x = map_range(x_2[0] as i32, (0, 255), (0, screen_width));
        joystick_2_position.y = map_range(y_2[0] as i32, (0, 255), (0, screen_height));

        display.clear();

        Circle::new(joystick_1_position, 10)
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 2))
            .draw(&mut display.device)
            .unwrap();

        Rectangle::new(joystick_2_position, Size::new(10, 10))
            .into_styled(PrimitiveStyle::with_fill(BinaryColor::On))
            .draw(&mut display.device)
            .unwrap();

        display.flush()?;
    }
}
