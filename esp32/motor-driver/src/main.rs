use std::process::Command;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use esp32_nimble::{BLEDevice, NimbleProperties, uuid128};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Input, InputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::ledc::{LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::prelude::{Hertz, Peripherals};

#[derive(Debug, PartialEq)]
enum ForwardDirection {
    Sleep,
    Stop,
    Break,
    Forward,
    Backwards,
}

#[derive(Debug, PartialEq)]
enum HorizontalDirection {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
struct State {
    forward_direction: ForwardDirection,
    horizontal_direction: HorizontalDirection,
    power_percentage: u8,
    max_power: f32,
    minimum_power: f32,
}

struct Motor<'d, EEP: OutputPin, FAULT: InputPin> {
    state: State,
    eep: PinDriver<'d, EEP, Output>,
    fault: PinDriver<'d, FAULT, Input>,
    timer: LedcTimerDriver<'d>,
    left0: LedcDriver<'d>,
    left1: LedcDriver<'d>,
    right0: LedcDriver<'d>,
    right1: LedcDriver<'d>,
}

impl<'d, EEP: OutputPin, FAULT: InputPin> Motor<'d, EEP, FAULT> {
    fn new(
        eep: impl Peripheral<P=EEP> + 'd,
        fault: impl Peripheral<P=FAULT> + 'd,
        in1: impl Peripheral<P=impl OutputPin> + 'd,
        in2: impl Peripheral<P=impl OutputPin> + 'd,
        in3: impl Peripheral<P=impl OutputPin> + 'd,
        in4: impl Peripheral<P=impl OutputPin> + 'd,
        timer: impl Peripheral<P=impl LedcTimer> + 'd,
        channel1: impl Peripheral<P=impl LedcChannel> + 'd,
        channel2: impl Peripheral<P=impl LedcChannel> + 'd,
        channel3: impl Peripheral<P=impl LedcChannel> + 'd,
        channel4: impl Peripheral<P=impl LedcChannel> + 'd,
    ) -> anyhow::Result<Self> {
        let timer = LedcTimerDriver::new(
            timer,
            &TimerConfig::new().frequency(Hertz(100_000).into()),
        )?;

        let mut left0 = LedcDriver::new(channel1, &timer, in1)?;
        let mut left1 = LedcDriver::new(channel2, &timer, in2)?;

        let mut right0 = LedcDriver::new(channel3, &timer, in3)?;
        let mut right1 = LedcDriver::new(channel4, &timer, in4)?;

        let eep = PinDriver::output(eep)?;
        let fault = PinDriver::input(fault)?;

        Ok(
            Self {
                state: State {
                    max_power: left0.get_max_duty() as f32,
                    minimum_power: 178.0,
                    power_percentage: 80,
                    forward_direction: ForwardDirection::Sleep,
                    horizontal_direction: HorizontalDirection::Center,
                },
                eep,
                fault,
                timer,
                left0,
                left1,
                right0,
                right1,
            }
        )
    }

    fn start(&mut self) -> anyhow::Result<()> {
        self.state.forward_direction = ForwardDirection::Stop;
        self.update()
    }

    fn get_power_percentage(&self) -> u32 {
        let percentage = self.state.power_percentage.clamp(0, 100) as f32 / 100.0;
        let min = self.state.minimum_power;
        let max = self.state.max_power;

        (percentage * (max - min) + min) as u32
    }

    fn forward(&mut self) -> anyhow::Result<()> {
        self.state.forward_direction = ForwardDirection::Forward;
        self.update()
    }

    fn left(&mut self) -> anyhow::Result<()> {
        self.state.horizontal_direction = HorizontalDirection::Left;
        self.update()
    }

    fn right(&mut self) -> anyhow::Result<()> {
        self.state.horizontal_direction = HorizontalDirection::Right;
        self.update()
    }

    fn center(&mut self) -> anyhow::Result<()> {
        self.state.horizontal_direction = HorizontalDirection::Center;
        self.update()
    }

    fn backwards(&mut self) -> anyhow::Result<()> {
        self.state.forward_direction = ForwardDirection::Backwards;
        self.update()
    }

    fn stop(&mut self) -> anyhow::Result<()> {
        self.state.forward_direction = ForwardDirection::Stop;
        self.update()
    }

    fn sleep(&mut self) -> anyhow::Result<()> {
        self.state.forward_direction = ForwardDirection::Sleep;
        self.update()
    }

    fn is_faulty(&self) -> bool {
        self.fault.is_low()
    }

    fn update(&mut self) -> anyhow::Result<()> {
        println!("{:?}", self.state);

        if self.state.forward_direction == ForwardDirection::Sleep {
            self.eep.set_low()?;
        } else {
            self.eep.set_high()?;
        }

        let current_power = self.get_power_percentage();

        println!("current_power: {}", current_power);

        let (mut left0, mut left1, mut right0, mut right1) = match self.state.forward_direction {
            ForwardDirection::Forward => (current_power, 0, current_power, 0),
            ForwardDirection::Backwards => (0, current_power, 0, current_power),
            ForwardDirection::Break => (current_power, current_power, current_power, current_power),
            ForwardDirection::Stop | ForwardDirection::Sleep => (0, 0, 0, 0),
        };

        match self.state.horizontal_direction {
            HorizontalDirection::Left => {
                right1 = 0;
                right0 = 0;
            }
            HorizontalDirection::Center => {}
            HorizontalDirection::Right => {
                left0 = 0;
                left1 = 0;
            }
        }

        self.left0.set_duty(left0)?;
        self.left1.set_duty(left1)?;

        self.right0.set_duty(right0)?;
        self.right1.set_duty(right1)?;

        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let eep = peripherals.pins.gpio1;
    let fault = peripherals.pins.gpio2;

    let in1 = peripherals.pins.gpio18;
    let in2 = peripherals.pins.gpio17;
    let in3 = peripherals.pins.gpio44;
    let in4 = peripherals.pins.gpio43;

    let mut motor = Arc::new(Mutex::new(Motor::new(
        eep, fault,
        in1, in2, in3, in4,
        peripherals.ledc.timer1,
        peripherals.ledc.channel0,
        peripherals.ledc.channel1,
        peripherals.ledc.channel2,
        peripherals.ledc.channel3,
    )?));

    let mut motor1 = motor.clone();
    let mut motor2 = motor.clone();

    let ble_device = BLEDevice::take();
    let server = ble_device.get_server();
    let service = server.create_service(uuid128!("00000000-0000-0000-0000-000000000000"));

    let speed_characteristic = service.lock().create_characteristic(
        uuid128!("00000000-0000-0000-0000-000000000002"),
        NimbleProperties::WRITE,
    );

    speed_characteristic.lock().on_write(move |args| {
        let data = args.recv_data;

        let mut motor = motor1.lock().unwrap();

        motor.state.power_percentage = data[0];
        motor.update().unwrap();

        println!("Wrote to writable characteristic: {:?}", args.recv_data);
    });

    let control_characteristic = service.lock().create_characteristic(
        uuid128!("00000000-0000-0000-0000-000000000001"),
        NimbleProperties::WRITE,
    );

    control_characteristic.lock().on_write(move |args| {
        let data = args.recv_data;

        let mut motor = motor2.lock().unwrap();

        for command in data {
            let _ = match command {
                0x00 => motor.start(),
                0x01 => motor.forward(),
                0x02 => motor.backwards(),
                0x03 => motor.stop(),
                0x04 => motor.left(),
                0x05 => motor.right(),
                0x06 => motor.center(),
                0x07 => motor.sleep(),
                _ => Ok(())
            };
        }

        println!("Wrote to writable characteristic: {:?}", args.recv_data);
    });

    let ble_advertising = ble_device.get_advertising();
    ble_advertising.name("Hot Wheels");
    ble_advertising.start().unwrap();

    loop {

        // if motor.is_faulty() {
        //     println!("fault is low!");
        //     // left.disable()?;
        //     // right.disable()?;
        // }

        FreeRtos::delay_ms(100);
    }
}

