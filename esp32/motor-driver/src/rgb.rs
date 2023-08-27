use esp_idf_hal::gpio::OutputPin;
use esp_idf_hal::ledc::{LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::units::Hertz;

#[derive(Copy, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn clear() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn yellow() -> Self {
        Self { r: 255, g: 255, b: 0 }
    }

    pub fn red() -> Self {
        Self { r: 255, g: 0, b: 0 }
    }

    pub fn green() -> Self {
        Self { r: 0, g: 255, b: 0 }
    }

    pub fn blue() -> Self {
        Self { r: 0, g: 0, b: 255 }
    }
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        Self {
            r: ((value >> 16) & 0xFF) as u8,
            g: ((value >> 8) & 0xFF) as u8,
            b: (value & 0xFF) as u8,
        }
    }
}

pub struct RGBLed<'d> {
    red: LedcDriver<'d>,
    green: LedcDriver<'d>,
    blue: LedcDriver<'d>,
}

impl<'d> RGBLed<'d> {
    pub fn new(
        red: impl Peripheral<P=impl OutputPin> + 'd,
        green: impl Peripheral<P=impl OutputPin> + 'd,
        blue: impl Peripheral<P=impl OutputPin> + 'd,
        timer: impl Peripheral<P=impl LedcTimer> + 'd,
        channel1: impl Peripheral<P=impl LedcChannel> + 'd,
        channel2: impl Peripheral<P=impl LedcChannel> + 'd,
        channel3: impl Peripheral<P=impl LedcChannel> + 'd,
    ) -> anyhow::Result<Self> {
        let timer = LedcTimerDriver::new(
            timer,
            &TimerConfig::new().frequency(Hertz(1000).into()),
        )?;

        let red = LedcDriver::new(channel1, &timer, red)?;
        let green = LedcDriver::new(channel2, &timer, green)?;
        let blue = LedcDriver::new(channel3, &timer, blue)?;

        Ok(Self { red, green, blue })
    }

    pub fn set_color(&mut self, color: Color) -> anyhow::Result<()> {
        self.red.set_duty(color.r.into())?;
        self.green.set_duty(color.g.into())?;
        self.blue.set_duty(color.b.into())?;

        Ok(())
    }
}
