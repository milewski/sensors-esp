use std::sync::{Arc, Mutex};

use esp32_nimble::{BLEDevice, NimbleProperties, uuid128};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::PinDriver;
use esp_idf_hal::prelude::*;

use crate::motor::Motor;
use crate::rgb::{Color, RGBLed};

mod motor;
mod rgb;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    // This is needed to power on the Lily-Go T-Display S3 board directly from the 5v pin
    let mut power_on = PinDriver::output(peripherals.pins.gpio15)?;
    power_on.set_high()?;

    // For RGB Led
    let red = peripherals.pins.gpio3;
    let green = peripherals.pins.gpio10;
    let blue = peripherals.pins.gpio11;

    // For Motor Driver
    let eep = peripherals.pins.gpio1;
    let fault = peripherals.pins.gpio2;

    let in1 = peripherals.pins.gpio18;
    let in2 = peripherals.pins.gpio17;
    let in3 = peripherals.pins.gpio44;
    let in4 = peripherals.pins.gpio43;

    let motor = Arc::new(Mutex::new(Motor::new(
        eep, fault,
        in1, in2, in3, in4,
        peripherals.ledc.timer1,
        peripherals.ledc.channel0,
        peripherals.ledc.channel1,
        peripherals.ledc.channel2,
        peripherals.ledc.channel3,
    )?));

    let led = Arc::new(Mutex::new(RGBLed::new(
        red, green, blue,
        peripherals.ledc.timer0,
        peripherals.ledc.channel4,
        peripherals.ledc.channel5,
        peripherals.ledc.channel6,
    )?));

    let ble_device = BLEDevice::take();
    let server = ble_device.get_server();
    let service = server.create_service(uuid128!("00000000-0000-0000-0000-000000000000"));

    let control_characteristic = service.lock().create_characteristic(
        uuid128!("00000000-0000-0000-0000-000000000001"),
        NimbleProperties::WRITE_NO_RSP,
    );

    let motor_1 = motor.clone();

    control_characteristic.lock().on_write(move |arguments| {
        let data = arguments.recv_data.try_into().expect("unable to cast data");
        let mut motor = motor_1.lock().expect("failed to acquire lock");
        motor.update(data).expect("failed to update motor");
    });

    let ble_advertising = ble_device.get_advertising();
    ble_advertising.name("Hot Wheels");
    ble_advertising.start().unwrap();

    let led_1 = led.clone();
    let led_2 = led.clone();
    let led_3 = led.clone();

    led_3.lock().unwrap().set_color(Color::red())?;

    server.on_connect(move |_, connection| {
        println!("connected!: {:?}", connection);
        let mut rgb_led_1 = led_1.lock().expect("failed to acquire lock");
        rgb_led_1.set_color(Color::green()).expect("failed to set color");
    });

    server.on_disconnect(move |connection| {
        println!("disconnected!: {:?}", connection);
        let mut rgb_led_2 = led_2.lock().expect("failed to acquire lock");
        rgb_led_2.set_color(Color::red()).expect("failed to set color");
    });

    loop {
        if motor.lock().unwrap().is_faulty() {
            let mut rgb_led_3 = led_3.lock().expect("failed to acquire lock");
            rgb_led_3.set_color(Color::yellow())?;
            FreeRtos::delay_ms(200);
            rgb_led_3.set_color(Color::clear())?;
            FreeRtos::delay_ms(100);
            println!("Motor is faulty");
        }

        FreeRtos::delay_ms(100);
    }
}

