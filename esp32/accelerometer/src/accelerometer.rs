use esp_idf_hal::delay::BLOCK;
use esp_idf_hal::gpio::{InputPin, OutputPin, PinDriver};
use esp_idf_hal::i2c::{I2c, I2cDriver};
use esp_idf_hal::i2c::config::Config;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::units::Hertz;

enum PowerControl {
    Link = 0b00100000,
    AutoSleep = 0b00010000,
    Measure = 0b00001000,
    WakeUp8Hz = 0b00000000,
    WakeUp4Hz = 0b00000001,
    WakeUp2Hz = 0b00000010,
    WakeUp1Hz = 0b00000011,
}

impl Into<u8> for PowerControl {
    fn into(self) -> u8 {
        self as u8
    }
}

enum RegisterMap {
    DeviceId = 0x00,
    AxisXData0 = 0x32,
    PowerControl = 0x2D,
}

impl Into<u8> for RegisterMap {
    fn into(self) -> u8 {
        self as u8
    }
}

pub struct Accelerometer<'d> {
    address: u8,
    driver: I2cDriver<'d>,
}

impl<'d> Accelerometer<'d> {
    pub fn new(
        i2c: impl Peripheral<P=impl I2c> + 'd,
        sda: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        scl: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        cs: Option<impl Peripheral<P=impl InputPin + OutputPin> + 'd>,
    ) -> anyhow::Result<Accelerometer<'d>> {
        // You can also plug this pin directly to vcc to keep it high
        if let Some(cs) = cs {
            let mut cs = PinDriver::output(cs)?;
            cs.set_high()?;
        }

        let driver_config = Config::default().baudrate(Hertz(400_000));
        let mut driver = I2cDriver::new(i2c, sda, scl, &driver_config)?;

        Ok(Self { driver, address: 0x53 })
    }

    fn read(&mut self, register: RegisterMap) -> anyhow::Result<[u8; 8]> {
        let mut response = [0u8; 8];

        self.driver.write_read(self.address, &[register.into()], &mut response, BLOCK)?;

        Ok(response)
    }

    fn write(&mut self, register: RegisterMap, data: u8) -> anyhow::Result<()> {
        let mut response = [0u8; 1];

        self.driver.write_read(self.address, &[register.into(), data], &mut response, BLOCK)?;

        Ok(())
    }

    pub fn device_id(&mut self) -> anyhow::Result<u8> {
        Ok(self.read(RegisterMap::DeviceId)?[0])
    }

    pub fn start(&mut self) -> anyhow::Result<()> {
        self.write(RegisterMap::PowerControl, PowerControl::Measure.into())
    }

    pub fn acceleration(&mut self) -> anyhow::Result<(i16, i16, i16)> {
        let response = self.read(RegisterMap::AxisXData0)?;

        Ok((
            i16::from_le_bytes([response[0], response[1]]),
            i16::from_le_bytes([response[2], response[3]]),
            i16::from_le_bytes([response[4], response[5]]),
        ))
    }
}