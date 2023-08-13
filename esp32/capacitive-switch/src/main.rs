use std::sync::Mutex;

use anyhow::anyhow;
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{InterruptType, PinDriver};
use esp_idf_hal::prelude::*;
use esp_idf_sys::TaskHandle_t;
use rotary_encoder_embedded::Direction;
use crate::capacitive_sensor::CapacitiveSensor;

mod capacitive_sensor;

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().ok_or(anyhow!("failed to initialize peripherals"))?;

    // For Display
    let scl = peripherals.pins.gpio11;
    let sda = peripherals.pins.gpio12;

    // For Capacitive Sensor
    let mut one = PinDriver::input(peripherals.pins.gpio1)?;
    let mut two = PinDriver::input(peripherals.pins.gpio2)?;
    let three = peripherals.pins.gpio3;
    let four = peripherals.pins.gpio10;

    let main_task_handle: TaskHandle_t = esp_idf_hal::task::current().unwrap();
    //
    // let timer_conf = esp_idf_hal::timer::config::Config::new().auto_reload(true);
    // let mut timer = esp_idf_hal::timer::TimerDriver::new(peripherals.timer00, &timer_conf)?;
    //
    // timer.set_alarm(200)?;
    one.set_interrupt_type(InterruptType::NegEdge)?;
    one.enable_interrupt()?;

    two.set_interrupt_type(InterruptType::NegEdge)?;
    two.enable_interrupt()?;

    unsafe {

        one.subscribe(move || {
            let event_number = 1;
            esp_idf_hal::task::notify(main_task_handle, event_number);
        })?;

        two.subscribe(move || {
            let event_number = 2;
            esp_idf_hal::task::notify(main_task_handle, event_number);
        })?;

        // timer.subscribe(move || {
        //     let event_number = 42;
        //     esp_idf_hal::task::notify(main_task_handle, event_number);
        // })?;
    }

    // timer.enable_alarm(true)?;
    // timer.enable(true)?;

    // let display = TinyDisplay::new(peripherals.i2c0, sda, scl)?;

    loop {

        // Notify approach
        // The benefit with this approach over checking a global static variable is
        // that the scheduler can hold the task, and resume when signaled
        // so no spinlock is needed
        let event_id = esp_idf_hal::task::wait_notification(None);

        // Note that the println functions are to slow for 200us
        // Even if we just send one charachter we can not go below 1ms per msg
        // so we are missing some events here - but if they are evaluated without
        // printing them the maintask will be fast enough no problem
        if let Some(event) = event_id {
            println!("got event with the number {event} from ISR");
        }

        FreeRtos::delay_ms(1);
    }
}
