use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::Peripherals;
use tetrice::{BlockKind, Cell, Game};
use crate::matrix::Matrix;

mod matrix;


fn selector() -> BlockKind {
    BlockKind::all_as_array()[fastrand::usize(0..7)]
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let sck = peripherals.pins.gpio6;
    let cs = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio4;

    let mut matrix_display = Matrix::new(peripherals.spi2, sck, mosi, cs)?;
    matrix_display.initialize()?;

    let mut game = Game::new(8, 16, 3, Box::new(selector));

    let mut matrix = [0u8; 128];

    loop {
        FreeRtos::delay_ms(100);

        // for (index, row) in game.field().as_vec().iter().enumerate() {
        //     for (cell_index, cell) in row.iter().enumerate() {
        //         let cell = match cell {
        //             Cell::Empty => 0,
        //             Cell::Block(_) => 1,
        //             _ => 1
        //         };
        //
        //         matrix[index * 8 + cell_index] = cell;
        //     }
        // }

        matrix_display.write_data(&matrix)?;

        game.soft_drop();
        // game.save();
        // game.move_left();

        println!("{:?}", game.tetrimino());

        FreeRtos::delay_ms(500);
    }
}
