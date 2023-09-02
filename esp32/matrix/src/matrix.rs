use esp_idf_hal::gpio::{AnyIOPin, AnyOutputPin, InputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::*;
use esp_idf_hal::spi::{SpiAnyPins, SpiConfig, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::spi::config::DriverConfig;
use esp_idf_sys::EspError;

#[derive(Copy, Clone)]
enum RegisterAddressMap {
    Noop = 0x0,
    Digit0 = 0x1,
    Digit1 = 0x2,
    Digit2 = 0x3,
    Digit3 = 0x4,
    Digit4 = 0x5,
    Digit5 = 0x6,
    Digit6 = 0x7,
    Digit7 = 0x8,
    DecodeMode = 0x9,
    Intensity = 0xA,
    ScanLimit = 0xB,
    Shutdown = 0xC,
    DisplayTest = 0xF,
}

impl Into<u8> for RegisterAddressMap {
    fn into(self) -> u8 {
        self as u8
    }
}

#[derive(Copy, Clone)]
pub enum Intensity {
    OneThirtyTwo = 0x00,
    ThreeThirtyTwo = 0x01,
    FiveThirtyTwo = 0x02,
    SevenThirtyTwo = 0x03,
    NineThirtyTwo = 0x04,
    ElevenThirtyTwo = 0x05,
    ThirteenThirtyTwo = 0x06,
    FifteenThirtyTwo = 0x07,
    SeventeenThirtyTwo = 0x08,
    NineteenThirtyTwo = 0x09,
    TwentyOneThirtyTwo = 0x0A,
    TwentyThreeThirtyTwo = 0x0B,
    TwentyFiveThirtyTwo = 0x0C,
    TwentySevenThirtyTwo = 0x0D,
    TwentyNineThirtyTwo = 0x0E,
    ThirtyOneThirtyTwo = 0x0F,
}

impl Into<u8> for Intensity {
    fn into(self) -> u8 {
        self as u8
    }
}

enum ScanLimit {
    DisplayDigit0 = 0x00,
    DisplayDigit0And1 = 0x01,
    DisplayDigit0To2 = 0x02,
    DisplayDigit0To3 = 0x03,
    DisplayDigit0To4 = 0x04,
    DisplayDigit0To5 = 0x05,
    DisplayDigit0To6 = 0x06,
    DisplayDigit0To7 = 0x07,
}

impl Into<u8> for ScanLimit {
    fn into(self) -> u8 {
        self as u8
    }
}

enum DecodeMode {
    NoDecode = 0x00,
    CodeB = 0x01,
    CodeBC = 0x0F,
    CodeBDE = 0xFF,
}

impl Into<u8> for DecodeMode {
    fn into(self) -> u8 {
        self as u8
    }
}

enum Mode {
    Shutdown = 0x00,
    NormalOperation = 0x01,
}

impl Into<u8> for Mode {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct Matrix<'d, CS: OutputPin, const BUFFER_SIZE: usize, const DISPLAY_COUNT: usize> {
    spi: SpiDeviceDriver<'d, SpiDriver<'d>>,
    cs: PinDriver<'d, CS, Output>,
    cache: [u8; BUFFER_SIZE],
    is_dirty: bool,
}

impl<'d, CS: OutputPin, const BUFFER_SIZE: usize, const DISPLAY_COUNT: usize> Matrix<'d, CS, BUFFER_SIZE, DISPLAY_COUNT> {
    pub fn new(
        spi: impl Peripheral<P=impl SpiAnyPins> + 'd,
        sck: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        mosi: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        cs: impl Peripheral<P=CS> + 'd,
    ) -> anyhow::Result<Self> {
        let driver_config = DriverConfig::default();

        let driver = SpiDriver::new(spi, sck, mosi, None::<AnyIOPin>, &driver_config)?;
        let config = SpiConfig::default().baudrate(5.MHz().into());
        let mut spi = SpiDeviceDriver::new(driver, None::<AnyOutputPin>, &config)?;

        let mut cs = PinDriver::output(cs)?;

        assert_eq!(BUFFER_SIZE, 8 * 8 * DISPLAY_COUNT, "buffer size must be 8 * 8 * display count");

        Ok(Self { spi, cs, cache: [0u8; BUFFER_SIZE], is_dirty: false })
    }

    pub fn initialize(&mut self) -> Result<(), EspError> {
        self.write(RegisterAddressMap::Shutdown, Mode::NormalOperation.into())?;
        self.write(RegisterAddressMap::DecodeMode, DecodeMode::NoDecode.into())?;
        self.write(RegisterAddressMap::ScanLimit, ScanLimit::DisplayDigit0To7.into())?;
        self.write(RegisterAddressMap::Intensity, Intensity::ThirtyOneThirtyTwo.into())?;

        self.clear()?;

        Ok(())
    }

    pub fn flush(&mut self) -> Result<(), EspError> {
        self.write_data()
    }

    pub fn set(&mut self, index: usize, value: u8) {
        if let Some(data) = self.cache.get_mut(index) {
            *data = value;
        }
    }

    pub fn fill(&mut self) {
        self.cache.fill(0);
        self.is_dirty = true;
    }

    pub fn clear(&mut self) -> Result<(), EspError> {
        self.fill();
        self.flush()?;

        Ok(())
    }

    fn write_data(&mut self) -> Result<(), EspError> {
        if self.is_dirty == false {
            return Ok(());
        }

        for row in 0..8u8 {
            let mut buffer = [0u8; DISPLAY_COUNT];

            for column in 0..8u8 {
                let index = (column * 8 + row) as usize;

                for display in 0..DISPLAY_COUNT {
                    buffer[display] |= self.cache[(display * 8 * 8) + index] << (7 - column);
                }
            }

            self.cs.set_low()?;

            for display in 0..DISPLAY_COUNT {
                self.spi.write(&[row + 1, buffer[display]])?;
            }

            self.cs.set_high()?;
        }

        self.is_dirty = false;

        Ok(())
    }

    fn write(&mut self, register: RegisterAddressMap, value: u8) -> Result<(), EspError> {
        self.cs.set_low()?;

        for _ in 0..DISPLAY_COUNT {
            self.spi.write(&[register.into(), value])?;
        }

        self.cs.set_high()
    }
}