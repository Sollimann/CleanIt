#![allow(dead_code)]
use crate::roomba::packets::battery_packets::decode_battery_packets;
use std::io::{Read, Write};
use std::time::Duration;
use std::{io, thread};

pub fn duplex() {
    // Open the first serialport available
    let port_name = &serialport::available_ports().expect("No serial port")[0].port_name;
    let mut port = serialport::new(port_name, 115_200)
        .open()
        .expect("Failed to open serial port");

    // set robot in mode
    const START: u8 = 128_u8;
    const PASSIVE_MODE: u8 = 128_u8;
    const SAFE_MODE: u8 = 131_u8;
    const FULL: u8 = 132_u8;
    const STOP: u8 = 173_u8;

    // Write a buffer into this writer, returning how many bytes were written.
    // https://doc.rust-lang.org/nightly/std/io/trait.Write.html
    port.flush().unwrap();
    if let Err(e) = port.write(&[START]) {
        eprintln!("error: {}", e)
    }
    println!("Starting");
    thread::sleep(Duration::from_millis(1000));
    // println!("Setting mode");
    // port.write(&[FULL]);
    // thread::sleep(Duration::from_millis(1000));

    // Clone the port
    let mut clone = port.try_clone().expect("Failed to clone");

    // send out 4 bytes every 15 ms
    thread::spawn(move || loop {
        clone.flush().unwrap();
        thread::sleep(Duration::from_millis(2));
        clone
            .write_all(&[142, 3])
            .expect("Failed to write to serial port");
        thread::sleep(Duration::from_millis(2));
        clone.flush().unwrap();
        thread::sleep(Duration::from_millis(4));
    });

    // Read the response from the cloned port
    let mut buffer = [0u8; 10];
    let mut _count = 1;
    loop {
        thread::sleep(Duration::from_millis(10));
        match port.read(&mut buffer) {
            Ok(bytes_recvd) => {
                _count += 1;
                println!("count: {}", _count);
                println!("buffer size: {} bytes", bytes_recvd);
                println!("buffer content: {:?}", &buffer);
                if bytes_recvd == buffer.len() {
                    //decode_sensor_packets(buffer);
                    decode_battery_packets(buffer)
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("This is an error: {:?}", e),
        }
        port.flush().unwrap();
    }
}
