use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::Sender;
use std::thread::spawn;

use esp_idf_hal::rmt::TxRmtDriver;

use crate::song::Song;
use crate::songs::green_hill::GreenHill;
use crate::songs::super_mario_bros::SuperMarioBros;
use crate::songs::tetris::Tetris;
use crate::songs::the_lion_sleeps_tonight::TheLionSleepsTonight;

pub struct Player {
    pub current_track: usize,
    sender: Option<Sender<()>>,
    transmitter: Arc<Mutex<TxRmtDriver<'static>>>,
    is_playing: bool,
    songs: Vec<fn() -> Box<dyn Song>>,
}

impl Player {
    pub fn new(tx: TxRmtDriver<'static>) -> Self {
        Self {
            transmitter: Arc::new(Mutex::new(tx)),
            current_track: 0,
            is_playing: false,
            sender: None,
            songs: vec![
                || Box::new(GreenHill::new()),
                || Box::new(TheLionSleepsTonight::new()),
                || Box::new(SuperMarioBros::new()),
                || Box::new(Tetris::new()),
            ],
        }
    }

    pub fn play(&mut self) -> anyhow::Result<()> {
        self.is_playing = true;

        let transmitter = self.transmitter.clone();
        let (sender, receiver) = mpsc::channel();
        let song = self.songs.get_mut(self.current_track);

        if let Some(result) = song {
            let song = result.clone();
            self.sender = Some(sender);

            spawn(move || {
                if let Ok(mut transmitter) = transmitter.lock() {
                    song().play(&mut transmitter, receiver).unwrap();
                }
            });
        }

        Ok(())
    }

    pub fn stop(&mut self) -> anyhow::Result<()> {
        self.is_playing = false;

        if let Some(sender) = self.sender.take() {
            sender.send(())?;
        }

        Ok(())
    }

    pub fn next(&mut self) -> anyhow::Result<()> {
        if self.is_playing {
            self.stop()?
        }

        self.current_track = self.current_track.wrapping_add(1) % self.songs.len();
        self.play()
    }

    pub fn previous(&mut self) -> anyhow::Result<()> {
        if self.is_playing {
            self.stop()?
        }

        self.current_track = self.current_track.wrapping_sub(1) % self.songs.len();
        self.play()
    }
}