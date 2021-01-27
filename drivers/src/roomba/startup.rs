use serialport::SerialPort;
use std::alloc::Global;
use std::thread;
use std::time::Duration;

// set robot in mode
const START: u8 = 128_u8;
const PASSIVE_MODE: u8 = 128_u8;
const SAFE_MODE: u8 = 131_u8;
const FULL: u8 = 132_u8;
const STOP: u8 = 173_u8;

pub fn startup() -> Box<dyn SerialPort, Global> {
    // Open the first serialport available
    let port_name = &serialport::available_ports().expect("No serial port")[0].port_name;
    let mut port = serialport::new(port_name, 115_200)
        .timeout(Duration::from_millis(30))
        .open()
        .expect("Failed to open serial port");

    //port.set_timeout(Duration::from_millis(30));
    // Write a buffer into this writer, returning how many bytes were written.
    // https://doc.rust-lang.org/nightly/std/io/trait.Write.html
    port.flush().unwrap();
    port.write(&[START]);
    println!("Starting");
    thread::sleep(Duration::from_millis(1000));
    println!("Setting mode");
    port.write(&[FULL]);
    thread::sleep(Duration::from_millis(1000));

    return port;
}

pub fn shutdown(mut port: Box<dyn SerialPort, Global>) {
    port.write(&[STOP]);
    println!("Stopping");
    thread::sleep(Duration::from_millis(500));
}
