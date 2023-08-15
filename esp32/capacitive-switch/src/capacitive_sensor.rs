#![allow(dead_code)]

use std::fmt::{Display, Formatter};

use anyhow::anyhow;
use esp_idf_hal::gpio::{Input, InputPin, InterruptType, PinDriver};
use esp_idf_hal::task;
use esp_idf_sys::TaskHandle_t;

type Callback = Box<dyn Fn(&ButtonPressed) -> anyhow::Result<()>>;

#[derive(Debug, Copy, Clone)]
pub enum ButtonPressed {
    One,
    Two,
    Three,
    Four,
}

impl Display for ButtonPressed {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ButtonPressed::One => write!(formatter, "1"),
            ButtonPressed::Two => write!(formatter, "2"),
            ButtonPressed::Three => write!(formatter, "3"),
            ButtonPressed::Four => write!(formatter, "4"),
        }
    }
}

impl TryFrom<u32> for ButtonPressed {
    type Error = u32;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            x if x == ButtonPressed::One as u32 => Ok(ButtonPressed::One),
            x if x == ButtonPressed::Two as u32 => Ok(ButtonPressed::Two),
            x if x == ButtonPressed::Three as u32 => Ok(ButtonPressed::Three),
            x if x == ButtonPressed::Four as u32 => Ok(ButtonPressed::Four),
            _ => Err(value),
        }
    }
}

impl Into<u32> for ButtonPressed {
    fn into(self) -> u32 {
        self as u32
    }
}

pub struct CapacitiveSensor<'d, ONE: InputPin, TWO: InputPin, THREE: InputPin, FOUR: InputPin> {
    one: PinDriver<'d, ONE, Input>,
    two: PinDriver<'d, TWO, Input>,
    three: PinDriver<'d, THREE, Input>,
    four: PinDriver<'d, FOUR, Input>,
    handle: TaskHandle_t,
    callbacks: Vec<Callback>,
}

impl<'d, ONE: InputPin, TWO: InputPin, THREE: InputPin, FOUR: InputPin> CapacitiveSensor<'d, ONE, TWO, THREE, FOUR> {
    pub fn new(one: ONE, two: TWO, three: THREE, four: FOUR) -> anyhow::Result<CapacitiveSensor<'d, ONE, TWO, THREE, FOUR>> {
        let mut one = PinDriver::input(one)?;
        let mut two = PinDriver::input(two)?;
        let mut three = PinDriver::input(three)?;
        let mut four = PinDriver::input(four)?;

        one.set_interrupt_type(InterruptType::PosEdge)?;
        two.set_interrupt_type(InterruptType::PosEdge)?;
        three.set_interrupt_type(InterruptType::PosEdge)?;
        four.set_interrupt_type(InterruptType::PosEdge)?;

        let handle = task::current().ok_or(anyhow!("failed to get current task"))?;

        unsafe {
            one.subscribe(move || {
                task::notify(handle, ButtonPressed::One.into());
            })?;

            two.subscribe(move || {
                task::notify(handle, ButtonPressed::Two.into());
            })?;

            three.subscribe(move || {
                task::notify(handle, ButtonPressed::Three.into());
            })?;

            four.subscribe(move || {
                task::notify(handle, ButtonPressed::Four.into());
            })?;
        }

        Ok(Self { one, two, three, four, handle, callbacks: vec![] })
    }

    pub fn on_touch(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        if let Some(event) = task::wait_notification(None) {
            if let Ok(button) = ButtonPressed::try_from(event) {
                for callback in &self.callbacks {
                    callback(&button)?;
                }
            }
        }

        Ok(())
    }
}