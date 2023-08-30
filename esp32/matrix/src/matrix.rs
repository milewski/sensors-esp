use esp_idf_hal::gpio::{AnyIOPin, InputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::spi::{SpiAnyPins, SpiConfig, SpiDeviceDriver, SpiDriver};
use esp_idf_hal::spi::config::DriverConfig;
use esp_idf_sys::EspError;

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

enum Intensity {
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

pub struct Matrix<'d, CS: OutputPin> {
    spi: SpiDeviceDriver<'d, SpiDriver<'d>>,
    cs: PinDriver<'d, CS, Output>,
}

impl<'d, CS: OutputPin> Matrix<'d, CS> {
    pub fn new(
        spi: impl Peripheral<P=impl SpiAnyPins> + 'd,
        sck: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        mosi: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        cs: impl Peripheral<P=CS> + 'd,
    ) -> anyhow::Result<Self> {
        let driver_config = DriverConfig::default();

        let driver = SpiDriver::new(spi, sck, mosi, Option::<AnyIOPin>::None, &driver_config)?;
        let spi_config = SpiConfig::default();
        let mut spi = SpiDeviceDriver::new(driver, Option::<AnyIOPin>::None, &spi_config)?;

        let mut cs = PinDriver::output(cs)?;

        Ok(Self { spi, cs })
    }

    pub fn initialize(&mut self) -> Result<(), EspError> {
        self.write(RegisterAddressMap::Shutdown, Mode::NormalOperation.into())?;
        self.write(RegisterAddressMap::DecodeMode, DecodeMode::NoDecode.into())?;
        self.write(RegisterAddressMap::ScanLimit, ScanLimit::DisplayDigit0To7.into())?;
        self.write(RegisterAddressMap::Intensity, Intensity::TwentyThreeThirtyTwo.into())?;
        self.clear()?;

        Ok(())
    }

    fn write(&mut self, register: RegisterAddressMap, value: u8) -> Result<(), EspError> {
        self.cs.set_high()?;
        self.spi.write(&[register.into(), value])?;
        self.cs.set_high()
    }

    pub fn clear(&mut self) -> Result<(), EspError> {
        for index in 1..=8 {
            self.cs.set_low()?;
            self.spi.write(&[index, 0])?;
            self.cs.set_high()?;
        }

        Ok(())
    }

    pub fn write_data(&mut self, matrix: &[u8; 128]) -> Result<(), EspError> {
        self.clear()?;

        for row in 0..8u8 {
            let mut display_top: u8 = 0;
            let mut display_bottom: u8 = 0;

            for column in 0..8 {
                let index = (column * 8 + row) as usize;

                // The data format is 0b00000000 where each bit represents a pixel (LED)
                display_top |= matrix[0 + index] << (7 - column);
                display_bottom |= matrix[64 + index] << (7 - column);
            }

            self.cs.set_low()?;

            // first write sends the data to the latest matrix on the chain
            self.spi.write(&[row + 1, display_top])?;
            self.spi.write(&[row + 1, display_bottom])?;

            self.cs.set_high()?;
        }

        Ok(())
    }
}