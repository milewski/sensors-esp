use button_driver::{Button, ButtonConfig};
use esp_idf_hal::gpio::{Input, InputPin, PinDriver};
pub use rotary_encoder_embedded::Direction;
use rotary_encoder_embedded::standard::StandardMode;

type Callback = Box<dyn Fn(Direction, bool) -> anyhow::Result<()>>;

pub struct RotaryEncoder<'d, CLK: InputPin, DT: InputPin, KEY: InputPin> {
    callbacks: Vec<Callback>,
    button: Option<Button<PinDriver<'d, KEY, Input>>>,
    encoder: rotary_encoder_embedded::RotaryEncoder<StandardMode, PinDriver<'d, DT, Input>, PinDriver<'d, CLK, Input>>,
}

impl<'d, CLK: InputPin, DT: InputPin, KEY: InputPin> RotaryEncoder<'d, CLK, DT, KEY> {
    pub fn new(
        s1_pin: CLK,
        s2_pin: DT,
        key_pin: Option<KEY>,
    ) -> anyhow::Result<RotaryEncoder<'d, CLK, DT, KEY>> {
        let clk = PinDriver::input(s1_pin)?;
        let dt = PinDriver::input(s2_pin)?;

        let mut button = None;

        if let Some(key_pin) = key_pin {
            button = Some(Button::new(PinDriver::input(key_pin)?, ButtonConfig::default()));
        }

        Ok(
            Self {
                button,
                encoder: rotary_encoder_embedded::RotaryEncoder::new(dt, clk).into_standard_mode(),
                callbacks: vec![],
            }
        )
    }

    pub fn handle(&mut self, callback: Callback) {
        self.callbacks.push(callback);
    }

    pub fn update(&mut self) -> anyhow::Result<()> {
        if let Some(button) = &mut self.button {
            button.tick();
        }

        self.encoder.update();

        for callback in &self.callbacks {
            callback(self.direction(), self.is_clicked())?;
        }

        self.reset();

        Ok(())
    }

    pub fn direction(&self) -> Direction {
        self.encoder.direction()
    }

    pub fn is_clicked(&self) -> bool {
        match &self.button {
            None => false,
            Some(button) => button.is_clicked()
        }
    }

    pub fn reset(&mut self) {
        if let Some(button) = &mut self.button {
            button.reset()
        }
    }
}