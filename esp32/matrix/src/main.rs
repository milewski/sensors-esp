use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::Peripherals;
use crate::matrix::Matrix;

mod matrix;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let sck = peripherals.pins.gpio6;
    let cs = peripherals.pins.gpio5;
    let mosi = peripherals.pins.gpio4;

    let mut matrix_display = Matrix::new(peripherals.spi2, sck, mosi, cs)?;
    matrix_display.initialize()?;

    let mut matrix = [0u8; 128];

    loop {
        FreeRtos::delay_ms(100);

        for index in 0..128 {
            matrix[index] = 1;
            matrix_display.write_data(&matrix)?;

            FreeRtos::delay_ms(25);
        }
    }
}
