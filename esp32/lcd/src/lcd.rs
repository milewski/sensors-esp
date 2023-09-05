use std::ops::BitOr;
use esp_idf_hal::delay;

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

#[derive(Copy, Clone)]
pub enum Backlight {
    Off = 0x00,
    On = 0x08,
}

#[derive(Copy, Clone)]
enum Mode {
    Command = 0b0000_0000,
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

enum Command {
    Clear = 0b0000_0001,
    ReturnHome = 0b0000_0010,
    ShiftCursor = 0b0001_0000,
}

#[derive(Copy, Clone)]
pub enum Direction {
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

pub struct LCD<'d, DELAY> {
    address: u8,
    delay: DELAY,
    backlight: Backlight,
    show_cursor: bool,
    blink_cursor: bool,
    driver: I2cDriver<'d>,
}

impl<'d, DELAY> LCD<'d, DELAY> where DELAY: Fn(u32) -> () {
    pub fn new(
        i2c: impl Peripheral<P=impl I2c> + 'd,
        sda: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        scl: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        delay: DELAY,
    ) -> Result<Self, EspError> {
        Ok(
            Self {
                delay,
                show_cursor: false,
                blink_cursor: false,
                address: 0b0100_111,
                backlight: Backlight::On,
                driver: I2cDriver::new(i2c, sda, scl, &Config::new().baudrate(100.kHz().into()))?,
            }
        )
    }

    pub fn backlight(&mut self, state: Backlight) -> Result<(), EspError> {
        self.backlight = state;
        self.write_command(0, Mode::DisplayControl)?;

        Ok(())
    }

    pub fn write_str(&mut self, message: &str) -> Result<(), EspError> {
        for char in message.chars().map(|char| char as u8) {
            self.write_command(char, Mode::Data)?;
        }

        Ok(())
    }

    pub fn scroll(&mut self, direction: Direction) -> Result<(), EspError> {
        self.write_command(Command::ShiftCursor as u8 | Push::Push as u8 | direction as u8, Mode::Command)?;

        Ok(())
    }

    pub fn initialize(&mut self) -> Result<(), EspError> {
        // Init with 8 bit mode
        let mode_8bit: u8 = Mode::FunctionSet as u8 | BitMode::Bit8 as u8;
        self.write_4_bits(mode_8bit)?;
        self.write_4_bits(mode_8bit)?;

        // Switch to 4 bit mode
        let mode_4bit = Mode::FunctionSet as u8 | BitMode::Bit4 as u8;
        self.write_4_bits(mode_4bit)?;

        // Function set command
        self.write_command(Mode::FunctionSet | Rows::TwoLines | Font::FiveByEleven, Mode::Command)?;

        let initialization_code = 0
            | DisplayControl::DisplayOn as u8
            | DisplayControl::CursorOn as u8
            | DisplayControl::CursorBlink as u8;

        self.write_command(Mode::DisplayControl as u8 | initialization_code, Mode::Command)?;

        self.cursor(false)?;
        self.cursor_blink(false)?;
        self.reset()?;

        Ok(())
    }

    pub fn cursor(&mut self, state: bool) -> Result<(), EspError> {
        self.show_cursor = state;

        let mut data = DisplayControl::DisplayOn as u8;

        if self.blink_cursor {
            data |= DisplayControl::CursorBlink as u8
        }

        if self.show_cursor {
            data |= DisplayControl::CursorOn as u8
        }

        self.write_command(Mode::DisplayControl as u8 | data, Mode::Command)?;

        Ok(())
    }

    pub fn cursor_blink(&mut self, state: bool) -> Result<(), EspError> {
        self.blink_cursor = state;
        self.cursor(self.show_cursor)?;

        Ok(())
    }

    fn write_4_bits(&mut self, data: u8) -> Result<(), EspError> {
        self.driver.write(self.address, &[data | DisplayControl::DisplayOn as u8 | self.backlight as u8], BLOCK)?;
        (self.delay)(1);

        self.driver.write(self.address, &[DisplayControl::Off as u8 | self.backlight as u8], BLOCK)?;
        (self.delay)(5);

        Ok(())
    }

    fn write_command(&mut self, data: u8, mode: Mode) -> Result<(), EspError> {
        let high_bits: u8 = data & 0b1111_0000;
        let low_bits: u8 = (data << 4) & 0b1111_0000;

        self.write_4_bits(high_bits | mode as u8)?;
        self.write_4_bits(low_bits | mode as u8)?;

        Ok(())
    }

    pub fn clear(&mut self) -> Result<(), EspError> {
        self.write_command(Command::Clear as u8, Mode::Command)?;

        Ok(())
    }

    pub fn reset(&mut self) -> Result<(), EspError> {
        self.clear()?;
        self.cursor_move_to(0, 0)?;

        Ok(())
    }

    pub fn cursor_move_to(&mut self, row: u8, column: u8) -> Result<(), EspError> {
        self.write_command(Command::ReturnHome as u8, Mode::Command)?;

        let shift: u8 = row * 40 + column;

        for _ in 0..shift {
            self.write_command(Command::ShiftCursor as u8 | Direction::Right as u8, Mode::Command)?;
        }

        Ok(())
    }
}