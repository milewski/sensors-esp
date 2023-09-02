use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::Peripherals;

use crate::matrix::Matrix;

mod matrix;

struct Tetrimino {
    width: usize,
    height: usize,
    shape: Vec<u8>,
    position: Position,
}

impl Tetrimino {
    fn new_l() -> Self {
        Self {
            width: 2,
            height: 3,
            position: Position { x: 0, y: 0 },
            shape: vec![
                1, 0,
                1, 0,
                1, 1,
            ],
        }
    }

    fn new_j() -> Self {
        Self {
            width: 2,
            height: 3,
            position: Position { x: 0, y: 0 },
            shape: vec![
                0, 1,
                0, 1,
                1, 1,
            ],
        }
    }

    fn new_t() -> Self {
        Self {
            width: 3,
            height: 2,
            position: Position { x: 0, y: 0 },
            shape: vec![
                0, 1, 0,
                1, 1, 1,
            ],
        }
    }

    fn new_o() -> Self {
        Self {
            width: 2,
            height: 2,
            position: Position { x: 0, y: 0 },
            shape: vec![
                1, 1,
                1, 1,
            ],
        }
    }

    fn new_s() -> Self {
        Self {
            width: 3,
            height: 2,
            position: Position { x: 0, y: 0 },
            shape: vec![
                0, 1, 1,
                1, 1, 0,
            ],
        }
    }

    fn new_z() -> Self {
        Self {
            width: 3,
            height: 2,
            position: Position { x: 0, y: 0 },
            shape: vec![
                1, 1, 0,
                0, 1, 1,
            ],
        }
    }

    fn new_i() -> Self {
        Self {
            width: 1,
            height: 4,
            position: Position { x: 0, y: 0 },
            shape: vec![
                1,
                1,
                1,
                1,
            ],
        }
    }

    fn new_random() -> Self {
        let mut block = match fastrand::u8(0..7) {
            0 => Self::new_l(),
            1 => Self::new_j(),
            2 => Self::new_t(),
            3 => Self::new_o(),
            4 => Self::new_s(),
            5 => Self::new_z(),
            6 => Self::new_i(),
            _ => unreachable!(),
        };

        for _ in 0..fastrand::u8(0..4) {
            block.rotate()
        }

        block.position.x = fastrand::usize(block.width - 1..8);
        block
    }

    fn rotate(&mut self) {
        let mut new_shape = vec![0; self.width * self.height];

        for height in 0..self.height {
            for width in 0..self.width {
                new_shape[width * self.height + self.height - height - 1] = self.shape[height * self.width + width];
            }
        }

        let old_width = self.width;
        let old_height = self.height;

        self.shape = new_shape;
        self.width = old_height;
        self.height = old_width;
    }
}

struct Position {
    x: usize,
    y: usize,
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let sck = peripherals.pins.gpio6;
    let cs = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio4;

    let mut display: Matrix<'_, _, 128, 2> = Matrix::new(peripherals.spi2, sck, mosi, cs)?;
    display.initialize()?;

    let mut block = Tetrimino::new_random();

    loop {
        display.fill();

        for width in 0..block.width {
            for height in 0..block.height {
                display.set(
                    (block.position.y + height) * 8 + (block.position.x.saturating_sub(width)),
                    block.shape[height * block.width + width],
                );
            }
        }

        block.position.y += 1;

        if block.position.y > 16 {
            block = Tetrimino::new_random();
        }

        display.flush()?;

        FreeRtos::delay_ms(50);
    }
}
