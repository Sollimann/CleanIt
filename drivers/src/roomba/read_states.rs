use crate::roomba::duplex::decode_sensor_packets;
use std::time::Duration;
use std::{io, thread};

pub fn sensors() {
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
    let mut msg = port.write(&[START]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));
    msg = port.write(&[FULL]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));
    port.write_all(&[142, 100])
        .expect("Failed to write to serial port");
    thread::sleep(Duration::from_millis(15));

    let mut buffer = [0u8; 80];
    let mut _count = 1;
    loop {
        thread::sleep(Duration::from_millis(15));
        match port.read(&mut buffer) {
            Ok(bytes_recvd) => {
                _count += 1;
                println!("count: {}", _count);
                println!("buffer size: {} bytes", bytes_recvd);
                println!("buffer content: {:?}", &buffer);
                if bytes_recvd == buffer.len() {
                    decode_sensor_packets(buffer)
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("This is an error: {:?}", e),
        }
    }
}
