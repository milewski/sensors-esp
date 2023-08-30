use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::Peripherals;
use esp_idf_hal::spi::config::DriverConfig;

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

    matrix[0] = 1;
    matrix[1] = 1;
    matrix[2] = 1;
    matrix[3] = 1;
    matrix[4] = 1;
    matrix[5] = 1;
    matrix[6] = 1;
    matrix[7] = 1;

    matrix[120] = 1;
    matrix[121] = 1;
    matrix[122] = 1;
    matrix[123] = 1;
    matrix[124] = 1;
    matrix[125] = 1;
    matrix[126] = 1;
    matrix[127] = 1;

    // Rotate Matrix

    loop {
        FreeRtos::delay_ms(100);

        matrix_display.write_data(&matrix)?;

        FreeRtos::delay_ms(100);
    }
}
