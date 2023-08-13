use std::time::Duration;
use button_driver::{Button, ButtonConfig, Mode};
use esp_idf_hal::gpio::{Input, InputPin, PinDriver};
use esp_idf_hal::peripheral::Peripheral;

type Callback = Box<dyn Fn(bool, bool, bool, bool) -> anyhow::Result<()>>;

pub struct CapacitiveSensor<'d, ONE: InputPin, TWO: InputPin, THREE: InputPin, FOUR: InputPin> {
    one: Button<PinDriver<'d, ONE, Input>>,
    two: Button<PinDriver<'d, TWO, Input>>,
    three: Button<PinDriver<'d, THREE, Input>>,
    four: Button<PinDriver<'d, FOUR, Input>>,
    callbacks: Vec<Callback>,
}

impl<'d, ONE, TWO, THREE, FOUR> CapacitiveSensor<'d, ONE, TWO, THREE, FOUR> where ONE: InputPin, TWO: InputPin, THREE: InputPin, FOUR: InputPin {
    pub fn new(one: ONE, two: TWO, three: THREE, four: FOUR) -> anyhow::Result<CapacitiveSensor<'d, ONE, TWO, THREE, FOUR>> {
        let pin_one = PinDriver::input(one)?;
        let pin_two = PinDriver::input(two)?;
        let pin_three = PinDriver::input(three)?;
        let pin_four = PinDriver::input(four)?;

        let mut one = Button::new(pin_one, ButtonConfig::default());
        let mut two = Button::new(pin_two, ButtonConfig::default());
        let mut three = Button::new(pin_three, ButtonConfig::default());
        let mut four = Button::new(pin_four, ButtonConfig::default());

        Ok(Self { one, two, three, four, callbacks: vec![] })
    }

    pub fn on_touch(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        self.one.tick();
        self.two.tick();
        self.three.tick();
        self.four.tick();

        for callback in &self.callbacks {
            callback(
                self.one.is_clicked(),
                self.two.is_clicked(),
                self.three.is_clicked(),
                self.four.is_clicked(),
            )?;
        }

        self.one.reset();
        self.two.reset();
        self.three.reset();
        self.four.reset();

        Ok(())
    }
}