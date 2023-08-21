use std::fmt::Display;
use std::ops::BitOr;

use anyhow::anyhow;
use esp_idf_hal::delay::{BLOCK, FreeRtos};
use esp_idf_hal::gpio::{InputPin, InterruptType, OutputPin, PinDriver};
use esp_idf_hal::i2c::{I2c, I2cDriver, Operation};
use esp_idf_hal::i2c::config::Config;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::task;
use esp_idf_hal::units::Hertz;

enum IntMap {
    DataReady = 0b10000000,
    SingleTap = 0b01000000,
    DoubleTap = 0b00100000,
    Activity = 0b00010000,
    Inactivity = 0b00001000,
    FreeFall = 0b00000100,
    Watermark = 0b00000010,
    Overrun = 0b00000001,
}

impl BitOr for IntMap {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl Into<u8> for IntMap {
    fn into(self) -> u8 {
        self as u8
    }
}

enum IntEnable {
    DataReady = 0b10000000,
    SingleTap = 0b01000000,
    DoubleTap = 0b00100000,
    Activity = 0b00010000,
    Inactivity = 0b00001000,
    FreeFall = 0b00000100,
    Watermark = 0b00000010,
    Overrun = 0b00000001,
}

impl Into<u8> for IntEnable {
    fn into(self) -> u8 {
        self as u8
    }
}

impl BitOr for IntEnable {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

impl BitOr<IntEnable> for u8 {
    type Output = u8;

    fn bitor(self, rhs: IntEnable) -> Self::Output {
        self | rhs as u8
    }
}

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

impl BitOr for PowerControl {
    type Output = u8;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u8 | rhs as u8
    }
}

enum RegisterMap {
    DeviceId = 0x00,
    OffsetX = 0x1E,
    OffsetY = 0x1F,
    OffsetZ = 0x20,
    AxisXData0 = 0x32,
    PowerControl = 0x2D,
    TapAxes = 0x2A,
    IntMap = 0x2F,
    IntSource = 0x30,
    IntEnable = 0x2E,
    ThreshTap = 0x1D,
    Dur = 0x21,
    Latent = 0x22,
    Window = 0x23,
    ActTapStatus = 0x2B,
    ActInactCtl = 0x27,
}

impl Into<u8> for RegisterMap {
    fn into(self) -> u8 {
        self as u8
    }
}

struct Accelerometer<'d> {
    address: u8,
    driver: I2cDriver<'d>,
}

impl<'d> Accelerometer<'d> {
    fn new(
        i2c: impl Peripheral<P=impl I2c> + 'd,
        sda: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
        scl: impl Peripheral<P=impl InputPin + OutputPin> + 'd,
    ) -> anyhow::Result<Accelerometer<'d>> {
        let driver_config = Config::default().baudrate(Hertz(400_000));
        let mut driver = I2cDriver::new(i2c, sda, scl, &driver_config)?;

        Ok(Self { driver, address: 0x53 })
    }

    fn access(&mut self, register: RegisterMap) -> anyhow::Result<u8> {
        let mut response = [0u8; 1];

        self.driver.write_read(0x53, &[register.into()], &mut response, BLOCK)?;

        Ok(response[0])
    }

    fn device_id(&mut self) -> anyhow::Result<u8> {
        self.access(RegisterMap::DeviceId)
    }

    fn offset_x(&mut self) -> anyhow::Result<u8> {
        self.access(RegisterMap::OffsetX)
    }

    fn offset_y(&mut self) -> anyhow::Result<u8> {
        self.access(RegisterMap::OffsetY)
    }

    fn offset_z(&mut self) -> anyhow::Result<u8> {
        self.access(RegisterMap::OffsetZ)
    }

    fn thresh_tap(&mut self, value: u8) -> anyhow::Result<[u8; 1]> {
        let mut response = [0u8; 1];
        self.driver.write_read(0x53, &[RegisterMap::ThreshTap.into(), value], &mut response, BLOCK)?;
        Ok(response)
    }

    fn dur(&mut self, value: u8) -> anyhow::Result<[u8; 1]> {
        let mut response = [0u8; 1];
        self.driver.write_read(0x53, &[RegisterMap::Dur.into(), value], &mut response, BLOCK)?;
        Ok(response)
    }

    fn latent(&mut self, value: u8) -> anyhow::Result<[u8; 1]> {
        let mut response = [0u8; 1];
        self.driver.write_read(0x53, &[RegisterMap::Latent.into(), value], &mut response, BLOCK)?;
        Ok(response)
    }

    fn window(&mut self, value: u8) -> anyhow::Result<[u8; 1]> {
        let mut response = [0u8; 1];
        self.driver.write_read(0x53, &[RegisterMap::Window.into(), value], &mut response, BLOCK)?;
        Ok(response)
    }

    fn int_map(&mut self, settings: u8) -> anyhow::Result<[u8; 1]> {
        let mut response = [0u8; 1];
        self.driver.write_read(0x53, &[RegisterMap::IntMap.into(), 0b00100000], &mut response, BLOCK)?;
        Ok(response)
    }

    fn int_source(&mut self) -> anyhow::Result<[u8; 8]> {
        let mut response = [RegisterMap::IntSource.into(); 8];

        // let operation = Operation::Read(&mut response);
        // self.driver.transaction(0x53, &mut [operation], BLOCK)?;

        self.driver.write_read(0x53, &[RegisterMap::IntSource.into()], &mut response, BLOCK)?;

        Ok(response)
    }

    fn tap_axes(&mut self) -> anyhow::Result<[u8; 1]> {
        let mut response = [0u8; 1];
        self.driver.write_read(0x53, &[RegisterMap::TapAxes.into(), 0b00000111], &mut response, BLOCK)?;
        Ok(response)
    }

    fn ack_tap_status(&mut self) -> anyhow::Result<[u8; 8]> {
        let mut response = [0u8; 8];
        self.driver.write_read(0x53, &[RegisterMap::ActTapStatus.into()], &mut response, BLOCK)?;
        Ok(response)
    }

    fn int_enable(&mut self, settings: u8) -> anyhow::Result<[u8; 1]> {
        let mut response = [0u8; 1];
        self.driver.write_read(0x53, &[RegisterMap::IntEnable.into(), settings], &mut response, BLOCK)?;
        Ok(response)
    }

    fn act_inact_ctl(&mut self) -> anyhow::Result<[u8; 12]> {
        let mut response = [0u8; 12];

        self.driver.write_read(0x53, &[RegisterMap::ActInactCtl.into(), 0xff], &mut response, BLOCK)?;

        Ok(response)
    }

    fn power_control(&mut self, settings: u8) -> anyhow::Result<[u8; 12]> {
        let mut response = [0u8; 12];

        self.driver.write_read(0x53, &[RegisterMap::PowerControl.into(), settings], &mut response, BLOCK)?;

        Ok(response)
    }

    fn acceleration(&mut self) -> anyhow::Result<(i16, i16, i16)> {
        let mut response = [0u8; 6];

        self.driver.write_read(0x53, &[RegisterMap::AxisXData0.into()], &mut response, BLOCK)?;

        Ok((
            i16::from_le_bytes([response[0], response[1]]),
            i16::from_le_bytes([response[2], response[3]]),
            i16::from_le_bytes([response[4], response[5]]),
        ))
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio43;
    let scl = peripherals.pins.gpio44;
    // let mut cs = PinDriver::output(peripherals.pins.gpio18)?;
    // cs.set_high()?;

    let mut interrupt_1 = PinDriver::input(peripherals.pins.gpio17)?;
    let mut interrupt_2 = PinDriver::input(peripherals.pins.gpio18)?;

    interrupt_1.set_interrupt_type(InterruptType::PosEdge)?;
    interrupt_2.set_interrupt_type(InterruptType::PosEdge)?;

    let handle = task::current().ok_or(anyhow!("failed to get current task")).unwrap();

    unsafe {
        interrupt_1.subscribe(move || {
            task::notify(handle, 1);
        }).unwrap();

        interrupt_2.subscribe(move || {
            task::notify(handle, 2);
        }).unwrap();
    }

    let mut accelerometer = Accelerometer::new(peripherals.i2c0, sda, scl)?;

    accelerometer.act_inact_ctl()?;
    accelerometer.power_control(PowerControl::Measure.into())?;
    accelerometer.int_enable(IntEnable::SingleTap | IntEnable::DoubleTap)?;
    accelerometer.int_map(IntMap::SingleTap.into())?;
    accelerometer.thresh_tap(100)?;
    accelerometer.tap_axes()?;
    accelerometer.dur(0x50)?;
    accelerometer.latent(0x60)?;
    accelerometer.window(0x90)?;

    println!("Device ID: 0x{:0X?}", accelerometer.device_id()?);

    loop {
        if let Some(event) = task::wait_notification(None) {
            let source = accelerometer.int_source();
            println!("Interrupt: {:x?}, source: {:?}", event, source);
        }

        FreeRtos::delay_ms(500);
    }
}

