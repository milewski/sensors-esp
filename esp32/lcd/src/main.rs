use anyhow::anyhow;
use esp_idf_hal::delay::{BLOCK, FreeRtos};
use esp_idf_hal::i2c::config::Config;
use esp_idf_hal::i2c::I2cDriver;
use esp_idf_hal::prelude::{FromValueType, Peripherals};

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let sda = peripherals.pins.gpio2;
    let scl = peripherals.pins.gpio1;

    let mut driver = I2cDriver::new(peripherals.i2c1, sda, scl, &Config::new().baudrate(100.kHz().into()))?;

    println!("{:08b}", 0x20 | 0x1 << 4 | 0x08);
    println!("{:08b}", 0x00 | 0x08);

    let address = 0b0100_111;

    // Init with 8 bit mode
    let mode_8bit = Mode::FunctionSet as u8 | BitMode::Bit8 as u8;
    write4bits(&mut driver, &address, mode_8bit);
    write4bits(&mut driver, &address, mode_8bit);
    // write4bits(&mut driver, &address, mode_8bit);

    // Switch to 4 bit mode
    let mode_4bit = Mode::FunctionSet as u8 | BitMode::Bit4 as u8;
    write4bits(&mut driver, &address, mode_4bit);

    // Function set command
    send(&mut driver, &address, Mode::FunctionSet as u8 | Rows::TwoLines as u8 | Font::FiveByEleven as u8, Mode::Cmd);
    // send(&mut driver, &address, Mode::FunctionSet as u8 | Font::FrontFiveByEight as u8, Mode::Cmd);

    let display_ctrl = DisplayControl::DisplayOn as u8 | DisplayControl::CursorOn as u8;
    let display_ctrl = display_ctrl | DisplayControl::CursorBlink as u8;

    send(&mut driver, &address, Mode::DisplayControl as u8 | display_ctrl, Mode::Cmd);
    send(&mut driver, &address, Mode::Cmd as u8 | Commands::Clear as u8, Mode::Cmd); // Clear Display

    for char in "Hello World!! I hope you dont mind if I".chars().map(|char| char as u8) {
        // send(&mut driver, &address, c as u8, Mode::Data);

        println!("{:08b} - {} ({:08b} - {:08b})", char, char, char & 0b1111_0000, (char << 4) & 0b1111_0000);

        let high_bits: u8 = char & 0b1111_0000;
        let low_bits: u8 = (char << 4) & 0b1111_0000;

        write4bits(&mut driver, &address, high_bits | Mode::Data as u8);
        write4bits(&mut driver, &address, low_bits | Mode::Data as u8);
    }

    set_cursor(&mut driver, &address, 1, 3);
    // send(&mut driver, &address, Commands::ReturnHome as u8, Mode::Cmd); // Clear Display
    // send(&mut driver, &address, Commands::ShiftCursor as u8, Mode::Cmd); // Clear Display
    // FreeRtos::delay_ms(2000);

        // send(&mut driver, &address, Commands::ShiftCursor as u8 | Push::Push as u8 | ShiftCursorDirection::Left as u8, Mode::Cmd); // Clear Display
    loop {
        FreeRtos::delay_ms(500);
    }
}

fn send(driver: &mut I2cDriver, address: &u8, data: u8, mode: Mode) {
    let high_bits: u8 = data & 0b1111_0000;
    let low_bits: u8 = (data << 4) & 0b1111_0000;
    write4bits(driver, address, high_bits | mode as u8);
    write4bits(driver, address, low_bits | mode as u8);
}

fn write4bits(driver: &mut I2cDriver, address: &u8, data: u8) {
    driver.write(
        *address,
        &[data | DisplayControl::DisplayOn as u8 | Backlight::On as u8],
        BLOCK,
    ).unwrap();
    FreeRtos::delay_ms(1);
    driver.write(
        *address,
        &[DisplayControl::Off as u8 | Backlight::On as u8],
        BLOCK,
    ).unwrap();
    FreeRtos::delay_ms(5);
}

fn set_cursor(driver: &mut I2cDriver, address: &u8, row: u8, col: u8) {
    send(driver, address, Commands::ReturnHome as u8, Mode::Cmd); // Clear Display
    let shift: u8 = row * 40 + col;
    for _ in 0..shift {
        send(driver, address, Commands::ShiftCursor as u8 | ShiftCursorDirection::Right as u8, Mode::Cmd); // Clear Display
    }
}

pub enum DisplayControl {
    Off = 0b0000_0000,
    // Defined as B on the datasheet
    CursorBlink = 0b0000_0001,
    // Defined as C on the datasheet
    CursorOn = 0b0000_0010,
    // Defined as D on the datasheet
    DisplayOn = 0b0000_0100,
}

pub enum Backlight {
    Off = 0x00,
    On = 0x08,
}

#[derive(Copy, Clone)]
enum Mode {
    Cmd = 0b0000_0000,
    Data = 0b0000_0001,
    DisplayControl = 0b0000_1000,
    FunctionSet = 0b0010_0000,
}

enum Commands {
    Clear = 0b0000_0001,
    ReturnHome = 0b0000_0010,
    ShiftCursor = 0b0001_0000,
}

enum ShiftCursorDirection {
    Left = 0b0000_0000,
    Right = 0b0000_0100,

}

enum Push {
    Push = 0b0000_1000,
}

enum BitMode {
    Bit4 = 0b0000_0000,
    Bit8 = 0b0001_0000,
}

enum Rows {
    OneLine = 0b0000_0000,
    TwoLines = 0b0000_1000,
}

enum Font {
    FiveByEleven = 0b0000_0100,
    FiveByEight = 0b0000_0000,
}