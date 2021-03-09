use byteorder::{BigEndian, WriteBytesExt};
use serialport::SerialPort;

pub fn drive(velocity: i16, radius: i16, mut port: Box<dyn SerialPort>) -> Box<dyn SerialPort> {
    let mut drive_commands: Vec<u8> = vec![137];
    drive_commands.write_i16::<BigEndian>(velocity).unwrap();
    drive_commands.write_i16::<BigEndian>(radius).unwrap();

    if let Err(e) = port.write(&drive_commands) {
        println!("writing drive commands failed due to error: {:?}", e)
    }
    port
}

pub fn drive_direct(
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
