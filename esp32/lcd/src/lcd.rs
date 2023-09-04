use std::ops::BitOr;
use esp_idf_hal::delay::{BLOCK, FreeRtos};
use esp_idf_hal::gpio::{InputPin, OutputPin};
use esp_idf_hal::i2c::{I2c, I2cDriver};
use esp_idf_hal::i2c::config::Config;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::FromValueType;
use esp_idf_sys::EspError;


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

impl BitOr<BitMode> for Mode {
    type Output = u8;

    fn bitor(self, rhs: BitMode) -> Self::Output {
        self as u8 | rhs as u8
    }
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

impl BitOr<Rows> for Mode {
    type Output = u8;

    fn bitor(self, rhs: Rows) -> Self::Output {
        self as u8 | rhs as u8
    }
}

enum Font {
    FiveByEleven = 0b0000_0100,
    FiveByEight = 0b0000_0000,
}

impl BitOr<Font> for Mode {
    type Output = u8;

    fn bitor(self, rhs: Font) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<Font> for u8 {
    type Output = u8;

    fn bitor(self, rhs: Font) -> Self::Output {
        self as u8 | rhs as u8
    }
}

pub struct LCD<'d> {
    address: u8,
    driver: I2cDriver<'d>,
}

impl<'d> LCD<'d> {
    pub fn new(
        i2c: impl Peripheral<P=impl I2c> + 'd,
        sda: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        scl: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
    ) -> Result<Self, EspError> {
        Ok(
            Self {
                address: 0b0100_111,
                driver: I2cDriver::new(i2c, sda, scl, &Config::new().baudrate(100.kHz().into()))?,
            }
        )
    }

    pub fn write_str(&mut self, message: &str) {
        for char in message.chars().map(|char| char as u8) {
            self.send(char, Mode::Data);
        }
    }

    pub fn initialize(&mut self) {

        // Init with 8 bit mode
        let mode_8bit: u8 = Mode::FunctionSet | BitMode::Bit8;
        self.write4bits(mode_8bit);
        self.write4bits(mode_8bit);
        // self.write4bits(mode_8bit);

        // Switch to 4 bit mode
        let mode_4bit = Mode::FunctionSet as u8 | BitMode::Bit4 as u8;
        self.write4bits(mode_4bit);

        // Function set command
        self.send(Mode::FunctionSet  | Rows::TwoLines  | Font::FiveByEleven , Mode::Cmd);
        // self.send(Mode::FunctionSet as u8 | Font::FrontFiveByEight as u8, Mode::Cmd);

        let display_ctrl = DisplayControl::DisplayOn as u8 | DisplayControl::CursorOn as u8;
        // let display_ctrl = display_ctrl | DisplayControl::CursorBlink as u8;

        self.send(Mode::DisplayControl as u8 | display_ctrl, Mode::Cmd);
        self.send(Mode::Cmd as u8 | Commands::Clear as u8, Mode::Cmd); // Clear Display
    }

    fn write4bits(&mut self, data: u8) {
        self.driver.write(
            self.address,
            &[data | DisplayControl::DisplayOn as u8 | Backlight::On as u8],
            BLOCK,
        ).unwrap();
        FreeRtos::delay_ms(1);
        self.driver.write(
            self.address,
            &[DisplayControl::Off as u8 | Backlight::On as u8],
            BLOCK,
        ).unwrap();
        FreeRtos::delay_ms(5);
    }

    fn send(&mut self, data: u8, mode: Mode) {
        let high_bits: u8 = data & 0b1111_0000;
        let low_bits: u8 = (data << 4) & 0b1111_0000;
        self.write4bits(high_bits | mode as u8);
        self.write4bits(low_bits | mode as u8);
    }

    fn set_cursor(&mut self, row: u8, col: u8) {
        self.send(Commands::ReturnHome as u8, Mode::Cmd); // Clear Display
        let shift: u8 = row * 40 + col;
        for _ in 0..shift {
            self.send(Commands::ShiftCursor as u8 | ShiftCursorDirection::Right as u8, Mode::Cmd); // Clear Display
        }
    }
}