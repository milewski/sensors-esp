use std::sync::mpsc::Receiver;

use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::rmt::{FixedLengthSignal, PinState, Pulse, PulseTicks, TxRmtDriver};

#[derive(Debug)]
pub struct Note(pub u16);

pub const B0: Note = Note(31);
pub const C1: Note = Note(33);
pub const CS1: Note = Note(35);
pub const D1: Note = Note(37);
pub const DS1: Note = Note(39);
pub const E1: Note = Note(41);
pub const F1: Note = Note(44);
pub const FS1: Note = Note(46);
pub const G1: Note = Note(49);
pub const GS1: Note = Note(52);
pub const A1: Note = Note(55);
pub const AS1: Note = Note(58);
pub const B1: Note = Note(62);
pub const C2: Note = Note(65);
pub const CS2: Note = Note(69);
pub const D2: Note = Note(73);
pub const DS2: Note = Note(78);
pub const E2: Note = Note(82);
pub const F2: Note = Note(87);
pub const FS2: Note = Note(93);
pub const G2: Note = Note(98);
pub const GS2: Note = Note(104);
pub const A2: Note = Note(110);
pub const AS2: Note = Note(117);
pub const B2: Note = Note(123);
pub const C3: Note = Note(131);
pub const CS3: Note = Note(139);
pub const D3: Note = Note(147);
pub const DS3: Note = Note(156);
pub const E3: Note = Note(165);
pub const F3: Note = Note(175);
pub const FS3: Note = Note(185);
pub const G3: Note = Note(196);
pub const GS3: Note = Note(208);
pub const A3: Note = Note(220);
pub const AS3: Note = Note(233);
pub const B3: Note = Note(247);
pub const C4: Note = Note(262);
pub const CS4: Note = Note(277);
pub const D4: Note = Note(294);
pub const DS4: Note = Note(311);
pub const E4: Note = Note(330);
pub const F4: Note = Note(349);
pub const FS4: Note = Note(370);
pub const G4: Note = Note(392);
pub const GS4: Note = Note(415);
pub const A4: Note = Note(440);
pub const AS4: Note = Note(466);
pub const B4: Note = Note(494);
pub const C5: Note = Note(523);
pub const CS5: Note = Note(554);
pub const D5: Note = Note(587);
pub const DS5: Note = Note(622);
pub const E5: Note = Note(659);
pub const F5: Note = Note(698);
pub const FS5: Note = Note(740);
pub const G5: Note = Note(784);
pub const GS5: Note = Note(831);
pub const A5: Note = Note(880);
pub const AS5: Note = Note(932);
pub const B5: Note = Note(988);
pub const C6: Note = Note(1047);
pub const CS6: Note = Note(1109);
pub const D6: Note = Note(1175);
pub const DS6: Note = Note(1245);
pub const E6: Note = Note(1319);
pub const F6: Note = Note(1397);
pub const FS6: Note = Note(1480);
pub const G6: Note = Note(1568);
pub const GS6: Note = Note(1661);
pub const A6: Note = Note(1760);
pub const AS6: Note = Note(1865);
pub const B6: Note = Note(1976);
pub const C7: Note = Note(2093);
pub const CS7: Note = Note(2217);
pub const D7: Note = Note(2349);
pub const DS7: Note = Note(2489);
pub const E7: Note = Note(2637);
pub const F7: Note = Note(2794);
pub const FS7: Note = Note(2960);
pub const G7: Note = Note(3136);
pub const GS7: Note = Note(3322);
pub const A7: Note = Note(3520);
pub const AS7: Note = Note(3729);
pub const B7: Note = Note(3951);
pub const C8: Note = Note(4186);
pub const CS8: Note = Note(4435);
pub const D8: Note = Note(4699);
pub const DS8: Note = Note(4978);
pub const REST: Note = Note(0);

pub trait Song {
    fn tempo(&self) -> f32;

    fn notes(&self) -> &[(Note, i8)];

    fn play(&mut self, tx: &mut TxRmtDriver<'static>, stop_signal: Receiver<()>) -> anyhow::Result<()> {
        let whole_note: f32 = (60000.0 * 4.0) / self.tempo();

        for (note, divider) in self.notes() {
            if let Ok(_) = stop_signal.try_recv() {
                return Ok(());
            }

            let mut note_duration: f32 = 0.0;

            if divider > &0 {
                note_duration = whole_note / *divider as f32;
            } else {
                note_duration = whole_note / divider.abs() as f32;
                note_duration *= 1.5;
            }

            self.play_pitch(tx, note.0, note_duration * 0.9)?;
        }

        Ok(())
    }

    fn play_pitch(&self, transmitter: &mut TxRmtDriver<'static>, pitch: u16, duration: f32) -> anyhow::Result<()> {
        if pitch == 0 {
            FreeRtos::delay_ms(duration as u32);
            return Ok(());
        }

        let ticks_hz = transmitter.counter_clock()?;
        let tick_count = (ticks_hz.0 / pitch as u32 / 2) as u16;
        let ticks = PulseTicks::new(tick_count)?;

        let on = Pulse::new(PinState::High, ticks);
        let off = Pulse::new(PinState::Low, ticks);
        let mut signal = FixedLengthSignal::<1>::new();

        signal.set(0, &(on, off))?;

        transmitter.start(signal)?;

        FreeRtos::delay_ms(duration as u32);

        transmitter.stop()?;

        Ok(())
    }
}
