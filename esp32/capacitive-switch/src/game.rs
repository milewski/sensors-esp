use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{Output, OutputPin, PinDriver};
use profont::{PROFONT_12_POINT, PROFONT_24_POINT};

use shared::tiny_display::TinyDisplay;

#[derive(Debug, PartialEq)]
enum State {
    Win,
    Lose,
    Playing,
}

struct GameState {
    secret: [u8; 4],
    buffer: [String; 4],
    index: usize,
}

impl GameState {
    fn new() -> Self {
        Self {
            secret: [fastrand::u8(1..=4), fastrand::u8(1..=4), fastrand::u8(1..=4), fastrand::u8(1..=4)],
            buffer: ["_", "__", "_", "_"].map(|value| value.to_string()),
            index: 0,
        }
    }

    fn update(&mut self, code: String, display: &mut TinyDisplay) -> anyhow::Result<State> {
        self.buffer[self.index] = code;
        self.index += 1;

        self.draw(display)?;

        if self.index == self.secret.len() {
            return self.check();
        }

        Ok(State::Playing)
    }

    fn check(&mut self) -> anyhow::Result<State> {
        let secret: [u8; 4] = self.buffer
            .iter()
            .map(|value| value.parse().unwrap())
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| anyhow!("failed to convert secret"))?;

        Ok(if secret == self.secret { State::Win } else { State::Lose })
    }

    fn draw(&mut self, display: &mut TinyDisplay) -> anyhow::Result<()> {
        for (index, code) in self.buffer.iter().enumerate() {
            display.draw_text(&code.to_string(), PROFONT_24_POINT, 25 + (20 * index as i32), 50)?;
        }

        display.flush()?;

        Ok(())
    }

    fn review_secret(&self) {
        println!("The secret code is: {}", self.secret.iter().fold(String::new(), |a, b| format!("{}{}", a, b)));
    }
}

pub struct Game<'a, LED: OutputPin> {
    state: GameState,
    led: PinDriver<'a, LED, Output>,
    display: TinyDisplay<'a>,
}

impl<'a, LED: OutputPin> Game<'a, LED> {
    pub fn new(display: TinyDisplay<'a>, led: PinDriver<'a, LED, Output>) -> anyhow::Result<Game<'a, LED>> {
        let mut game = Self { display, led, state: GameState::new() };

        game.initialize()?;

        Ok(game)
    }

    pub fn update(&mut self, code: String) -> anyhow::Result<()> {
        match self.state.update(code, &mut self.display)? {
            State::Win => self.draw_win_state(),
            State::Lose => self.draw_lose_state(),
            State::Playing => Ok(())
        }
    }

    fn initialize(&mut self) -> anyhow::Result<()> {
        self.display.clear();
        self.display
            .draw_text(&"Enter Code".to_string(), PROFONT_12_POINT, 25, 20)
            .map_err(|_| anyhow!("failed to lock display"))?;

        self.led.set_low()?;

        self.state.review_secret();
        self.state.draw(&mut self.display)
    }

    fn draw_win_state(&mut self) -> anyhow::Result<()> {
        self.display.clear();
        self.display.draw_text(&"Congratulations".to_string(), PROFONT_12_POINT, 5, 35)?;
        self.display.flush()?;
        self.led.set_high()?;
        self.reset(1000)
    }

    fn draw_lose_state(&mut self) -> anyhow::Result<()> {
        self.display.clear();
        self.display.draw_text(&"Try Again!".to_string(), PROFONT_12_POINT, 25, 35)?;
        self.display.flush()?;
        self.reset(500)
    }

    fn reset(&mut self, delay: u32) -> anyhow::Result<()> {
        FreeRtos::delay_ms(delay);
        self.state = GameState::new();
        self.initialize()
    }
}