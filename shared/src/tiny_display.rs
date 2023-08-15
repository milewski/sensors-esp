use anyhow::anyhow;
use embedded_graphics::mono_font::{MonoFont, MonoTextStyle};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use esp_idf_hal::gpio::{InputPin, OutputPin};
use esp_idf_hal::i2c::{I2c, I2cConfig, I2cDriver};
use esp_idf_hal::peripheral::Peripheral;
use ssd1306::*;
use ssd1306::mode::BufferedGraphicsMode;
use ssd1306::prelude::*;

pub struct TinyDisplay<'d> {
    device: Ssd1306<I2CInterface<I2cDriver<'d>>, DisplaySize128x64, BufferedGraphicsMode<DisplaySize128x64>>,
}

impl<'d> TinyDisplay<'d> {
    pub fn new<I2C: I2c>(
        i2c: impl Peripheral<P=I2C> + 'd,
        sda: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        scl: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
    ) -> anyhow::Result<TinyDisplay<'d>> {
        let config = I2cConfig::new();
        let driver = I2cDriver::new(i2c, sda, scl, &config)?;

        let interface: I2CInterface<I2cDriver> = I2CDisplayInterface::new(driver);

        let device = Ssd1306::new(
            interface,
            DisplaySize128x64,
            DisplayRotation::Rotate0,
        );

        let mut device = device.into_buffered_graphics_mode();

        device.init().map_err(|error| anyhow!("unable to initialize display: {:?}", error))?;
        device.clear_buffer();

        Ok(Self { device })
    }

    pub fn clear(&mut self) {
        self.device.clear_buffer();
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.device.flush().map_err(|error| anyhow!("failed to flush display: {:?}", error))
    }

    pub fn draw_text(&mut self, text: &String, font: MonoFont, x: i32, y: i32) -> anyhow::Result<Point> {
        let text = Text::new(
            text,
            Point::new(x, y),
            MonoTextStyle::new(&font, BinaryColor::On),
        );

        text.draw(&mut self.device)
            .map_err(|error| anyhow!("failed to draw to the display: {:?}", error))
    }
}
