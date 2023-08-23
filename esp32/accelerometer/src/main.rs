use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::prelude::Peripherals;
use crate::accelerometer::Accelerometer;

mod accelerometer;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let sda = peripherals.pins.gpio43;
    let scl = peripherals.pins.gpio44;

    let mut accelerometer = Accelerometer::new(
        peripherals.i2c0, sda, scl, Some(peripherals.pins.gpio21),
    )?;

    accelerometer.start()?;

    println!("Device ID: 0x{:0X?}", accelerometer.device_id()?);

    loop {
        println!("Acceleration: {:?}", accelerometer.acceleration()?);
        FreeRtos::delay_ms(10);
    }
}

