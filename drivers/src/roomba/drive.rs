use crate::roomba::packets::sensor_packets::decode_sensor_packets;
use crate::roomba::serial_stream::read_serial_stream;
use crate::roomba::startup::{shutdown, startup};
use byteorder::{BigEndian, WriteBytesExt};
use serialport::SerialPort;
use std::thread;
use std::time::Duration;

fn drive(velocity: i16, radius: i16, mut port: Box<dyn SerialPort>) -> Box<dyn SerialPort> {
    let mut drive_commands: Vec<u8> = vec![137];
    drive_commands.write_i16::<BigEndian>(velocity).unwrap();
    drive_commands.write_i16::<BigEndian>(radius).unwrap();

    if let Err(e) = port.write(&drive_commands) {
        println!("writing drive commands failed due to error: {:?}", e)
    }
    port
}

fn drive_direct(
    left_velocity: i16,
    right_velocity: i16,
    mut port: Box<dyn SerialPort>,
) -> Box<dyn SerialPort> {
    let mut drive_commands: Vec<u8> = vec![145];
    drive_commands
        .write_i16::<BigEndian>(right_velocity)
        .unwrap();
    drive_commands
        .write_i16::<BigEndian>(left_velocity)
        .unwrap();

    if let Err(e) = port.write(&drive_commands) {
        println!("writing drive direct commands failed due to error: {:?}", e)
    }
    port
}

pub fn drive_and_sense() {
    let mut port = startup();

    let clone = port.try_clone().expect("Failed to clone");

    // read sensor values in one thread
    thread::spawn(|| {
        read_serial_stream(clone, decode_sensor_packets); // 50hz
    });

    // drive the roomba in main thread
    //port = drive(100, 200, port);
    port = drive_direct(55, 55, port);
    thread::sleep(Duration::from_millis(5000));
    port = drive_direct(0, 0, port);
    thread::sleep(Duration::from_millis(1000));
    shutdown(port);
}
