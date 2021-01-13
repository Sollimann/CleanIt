use byteorder::{BigEndian, ByteOrder};
use std::io::{Cursor, Error, Write};
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
    const SAFE_MODE: u8 = 131_u8;
    const FULL: u8 = 132_u8;

    // Write a buffer into this writer, returning how many bytes were written.
    // https://doc.rust-lang.org/nightly/std/io/trait.Write.html
    let mut msg = port.write(&[FULL]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));
    msg = port.write(&[START]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));

    // Clone the port
    let mut clone = port.try_clone().expect("Failed to clone");

    // send out 4 bytes every 15 ms
    thread::spawn(move || loop {
        clone
            .write_all(&[142, 24])
            .expect("Failed to write to serial port");
        thread::sleep(Duration::from_millis(15))
    });

    // Read the response from the cloned port
    let mut buffer = [0u8; 10];
    let mut _count = 1;
    loop {
        match port.read(&mut buffer) {
            Ok(bytes_recvd) => {
                thread::sleep(Duration::from_millis(15));
                _count += 1;
                println!("count: {}", _count);
                println!("buffer size: {} bytes", bytes_recvd);
                println!("buffer content: {:?}", &buffer);
                let value1 = byteorder::BigEndian::read_i16(&buffer);
                println!("buffer decode: {}", value1);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("This is an error: {:?}", e),
        }
    }
}
