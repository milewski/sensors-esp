use dfr0299::{Command, RequestAck};
use esp_idf_hal::delay::FreeRtos;
use esp_idf_hal::gpio::{AnyInputPin, AnyOutputPin, PinDriver};
use esp_idf_hal::peripherals::Peripherals;
use esp_idf_hal::uart::config::Config;
use esp_idf_hal::uart::UartDriver;
use esp_idf_hal::units::Hertz;

fn write_command(driver: &UartDriver, command: Command, reply: RequestAck) {
    let mut buffer = [0u8; 10];

    if reply == RequestAck::Yes {
        command.serialise_with_ack(&mut buffer, reply).unwrap();
    } else {
        command.serialise(&mut buffer).unwrap();
    }

    driver.write(&mut buffer).unwrap();

    if reply == RequestAck::Yes {
        read_command(&driver, Command::Reply);
    }
}

fn read_command(driver: &UartDriver, command: Command) -> u8 {
    let mut buffer = [0u8; 10];
    let mut limit = 10;
    loop {
        command.serialise(&mut buffer).unwrap();
        let reply = driver.read(&mut buffer, 100).unwrap();
        limit -= 1;
        if limit == 0 {
            return 0;
        }
        if reply == 0 {
            println!("error: {:x?} {:?}", buffer, command);
            continue;
        }

        if buffer[3] == 0x40 {
            println!("Error retrying");
            continue;
        }

        if buffer[3] == 0x41 {
            continue;
        }

        if buffer[3] == 0x43 {
            println!("43: {:x?}", buffer);
            return buffer[6];
        }

        if buffer[3] == 0x3f {
            println!("3f: {:x?}", buffer);
            return buffer[6];
        }

        println!("buffer: {:x?}", buffer);
    }
}

fn main() -> anyhow::Result<()> {
    esp_idf_sys::link_patches();

    let peripherals = Peripherals::take().unwrap();

    let tx = peripherals.pins.gpio17;
    let rx = peripherals.pins.gpio18;
    let busy = PinDriver::input(peripherals.pins.gpio43)?;

    let driver = UartDriver::new(
        peripherals.uart1,
        rx,
        tx,
        Option::<AnyInputPin>::None,
        Option::<AnyOutputPin>::None,
        &Config::new().baudrate(Hertz(9600)),
    )?;

    write_command(&driver, Command::Reset, RequestAck::Yes);

    // write_command(&driver, Command::SetVolume(20), RequestAck::Yes);
    //
    // write_command(&driver, Command::Track(1), RequestAck::Yes);
    // println!("is_busy? {:?}", busy.is_high());
    //
    // FreeRtos::delay_ms(3000);
    // println!("is_busy? {:?}", busy.is_high());
    // write_command(&driver, Command::GetVolume, RequestAck::Yes);
    // println!("is_busy? {:?}", busy.is_high());
    //
    // println!("Donw@");

    loop {
        FreeRtos::delay_ms(100);
    }
}

