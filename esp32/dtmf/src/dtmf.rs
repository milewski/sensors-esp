use anyhow::anyhow;
use esp_idf_hal::gpio::{Input, InputPin, InterruptType, PinDriver};
use esp_idf_hal::task;

pub struct DTMF<'d, ONE: InputPin, TWO: InputPin, THREE: InputPin, FOUR: InputPin, ST: InputPin> {
    q1: PinDriver<'d, ONE, Input>,
    q2: PinDriver<'d, TWO, Input>,
    q3: PinDriver<'d, THREE, Input>,
    q4: PinDriver<'d, FOUR, Input>,
    st: PinDriver<'d, ST, Input>,
    callbacks: Vec<Box<dyn Fn(String) -> ()>>,
}

impl<'d, ONE: InputPin, TWO: InputPin, THREE: InputPin, FOUR: InputPin, ST: InputPin> DTMF<'d, ONE, TWO, THREE, FOUR, ST> {
    pub fn new(
        q1: ONE,
        q2: TWO,
        q3: THREE,
        q4: FOUR,
        st: ST,
    ) -> anyhow::Result<Self> {
        let q1 = PinDriver::input(q1)?;
        let q2 = PinDriver::input(q2)?;
        let q3 = PinDriver::input(q3)?;
        let q4 = PinDriver::input(q4)?;
        let mut st = PinDriver::input(st)?;

        st.set_interrupt_type(InterruptType::NegEdge)?;

        let handle = task::current().ok_or(anyhow!("failed to get current task"))?;

        unsafe {
            st.subscribe(move || {
                task::notify(handle, 0x01);
            })?;
        }

        Ok(Self { q1, q2, q3, q4, st, callbacks: vec![] })
    }

    fn read(&self) -> String {
        let mut number = 0b0000_0000;

        number |= if self.q1.is_low() { 0 } else { 1 } << 0;
        number |= if self.q2.is_low() { 0 } else { 1 } << 1;
        number |= if self.q3.is_low() { 0 } else { 1 } << 2;
        number |= if self.q4.is_low() { 0 } else { 1 } << 3;

        match number {
            0 => "D".to_string(),
            10 => "0".to_string(),
            11 => "*".to_string(),
            12 => "#".to_string(),
            13 => "A".to_string(),
            14 => "B".to_string(),
            15 => "C".to_string(),
            other => other.to_string()
        }
    }

    pub fn listen(&self) {
        loop {
            if let Some(_) = task::wait_notification(None) {
                for callback in &self.callbacks {
                    callback(self.read());
                }
            }
        }
    }

    pub fn on_pressed(&mut self, callback: Box<dyn Fn(String) -> ()>) {
        self.callbacks.push(callback)
    }
}
