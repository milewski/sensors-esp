use esp_idf_hal::gpio::{Input, InputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::ledc::{LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver};
use esp_idf_hal::ledc::config::TimerConfig;
use esp_idf_hal::peripheral::Peripheral;
use esp_idf_hal::units::Hertz;

#[derive(Debug, PartialEq)]
enum Direction {
    Forward,
    Backwards,
    Neutral,
}

#[derive(Debug)]
struct State {
    maximum_power: u32,
    minimum_power: u32,
    x_direction: Direction,
    x_power: u8,
    y_direction: Direction,
    y_power: u8,
}

impl State {
    fn update(&mut self, data: [u8; 4]) {
        self.x_power = data[1];
        self.y_power = data[3];

        self.x_direction = match data[0] {
            1 => Direction::Forward,
            2 => Direction::Backwards,
            _ => Direction::Neutral,
        };

        self.y_direction = match data[2] {
            1 => Direction::Forward,
            2 => Direction::Backwards,
            _ => Direction::Neutral,
        };
    }
}

pub struct Motor<'d, EEP: OutputPin, FAULT: InputPin> {
    state: State,
    eep: PinDriver<'d, EEP, Output>,
    fault: PinDriver<'d, FAULT, Input>,
    left0: LedcDriver<'d>,
    left1: LedcDriver<'d>,
    right0: LedcDriver<'d>,
    right1: LedcDriver<'d>,
}

impl<'d, EEP: OutputPin, FAULT: InputPin> Motor<'d, EEP, FAULT> {
    pub fn new(
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

        let left0 = LedcDriver::new(channel1, &timer, in1)?;
        let left1 = LedcDriver::new(channel2, &timer, in2)?;

        let right0 = LedcDriver::new(channel3, &timer, in3)?;
        let right1 = LedcDriver::new(channel4, &timer, in4)?;

        let eep = PinDriver::output(eep)?;
        let fault = PinDriver::input(fault)?;

        Ok(
            Self {
                state: State {
                    maximum_power: left0.get_max_duty(),
                    minimum_power: 150,
                    x_power: 0,
                    y_power: 0,
                    x_direction: Direction::Neutral,
                    y_direction: Direction::Neutral,
                },
                eep,
                fault,
                left0,
                left1,
                right0,
                right1,
            }
        )
    }

    fn get_power_percentage(&self, value: u8) -> u32 {
        let percentage = value as f32 / 100.0;
        let min = self.state.minimum_power as f32;
        let max = self.state.maximum_power as f32;

        (percentage * (max - min) + min) as u32
    }

    fn map(&self, value: i8, low1: i8, high1: i8, low2: i8, high2: i8) -> i8 {
        let value = value as i16;
        let low1 = low1 as i16;
        let low2 = low2 as i16;
        let high1 = high1 as i16;
        let high2 = high2 as i16;

        (low2 + (high2 - low2) * (value - low1) / (high1 - low1)) as i8
    }

    pub fn is_faulty(&self) -> bool {
        self.fault.is_low()
    }

    pub fn update(&mut self, data: [u8; 4]) -> anyhow::Result<()> {
        self.state.update(data);
        self.eep.set_high()?;

        let mut left_wheel: i8 = match self.state.y_direction {
            Direction::Forward => self.state.y_power as i8 * 1,
            Direction::Backwards => self.state.y_power as i8 * -1,
            Direction::Neutral => 0
        };

        let mut right_wheel: i8 = left_wheel.clone();

        let direction = match self.state.y_direction {
            Direction::Forward => 100,
            Direction::Backwards => -100,
            Direction::Neutral => 0
        };

        if self.state.x_direction == Direction::Backwards {
            left_wheel = self.map(self.state.x_power as i8, 0, 100, left_wheel, direction);
        }

        if self.state.x_direction == Direction::Forward {
            right_wheel = self.map(self.state.x_power as i8, 0, 100, right_wheel, direction);
        }

        let left_wheel_value = self.get_power_percentage(left_wheel.abs() as u8);
        let right_wheel_value = self.get_power_percentage(right_wheel.abs() as u8);

        self.left0.set_duty(if left_wheel.is_negative() { left_wheel_value } else { 0 })?;
        self.left1.set_duty(if left_wheel.is_positive() { left_wheel_value } else { 0 })?;

        self.right0.set_duty(if right_wheel.is_negative() { right_wheel_value } else { 0 })?;
        self.right1.set_duty(if right_wheel.is_positive() { right_wheel_value } else { 0 })?;

        Ok(())
    }
}
