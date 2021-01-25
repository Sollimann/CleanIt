use crate::roomba::decode::*;
use byteorder::{BigEndian, ByteOrder};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
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
    let mut msg = port.write(&[START]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));
    msg = port.write(&[FULL]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));

    // Clone the port
    let mut clone = port.try_clone().expect("Failed to clone");

    // send out 4 bytes every 15 ms
    thread::spawn(move || loop {
        clone
            .write_all(&[142, 100])
            .expect("Failed to write to serial port");
        thread::sleep(Duration::from_millis(15))
    });
    // clone
    //     .write_all(&[142, 23])
    //     .expect("Failed to write to serial port");

    // Read the response from the cloned port
    let mut buffer = [0u8; 80];
    let mut _count = 1;
    loop {
        match port.read(&mut buffer) {
            Ok(bytes_recvd) => {
                thread::sleep(Duration::from_millis(15));
                _count += 1;
                println!("count: {}", _count);
                println!("buffer size: {} bytes", bytes_recvd);
                println!("buffer content: {:?}", &buffer);
                println!("packet 58: {:?}", &buffer[79..]);
                let value1 = byteorder::BigEndian::read_i16(&buffer[79..]);
                println!("buffer decode: {}", value1);
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("This is an error: {:?}", e),
        }
    }
}

pub fn decode_sensor_packets(byte_data: Box<[u8]>) {
    #[derive(Debug)]
    enum Value {
        Str(String),
        Bool(bool),
        Int16(i16),
        Uint16(u16),
        Int8(i8),
        Uint8(u8),
        HashMap(HashMap<String, u8>),
    }

    let mut sensor_data = HashMap::new();

    // size 80, contains 7-58 (ALL)
    sensor_data.insert("stasis", Value::HashMap(decode_packet_58(byte_data[79])));

    sensor_data.insert(
        "side brush motor current",
        Value::Int16(decode_packet_57(byte_data[77], byte_data[78])),
    );
    sensor_data.insert(
        "main brush motor current",
        Value::Int16(decode_packet_56(byte_data[75], byte_data[76])),
    );
    sensor_data.insert(
        "right motor current",
        Value::Int16(decode_packet_55(byte_data[73], byte_data[74])),
    );
    sensor_data.insert(
        "left motor current",
        Value::Int16(decode_packet_54(byte_data[71], byte_data[72])),
    );
    sensor_data.insert(
        "infrared char right",
        Value::Uint8(decode_packet_53(byte_data[70])),
    );
    sensor_data.insert(
        "infrared char left",
        Value::Uint8(decode_packet_52(byte_data[69])),
    );
    sensor_data.insert(
        "light bump right signal",
        Value::Uint16(decode_packet_51(byte_data[67], byte_data[68])),
    );
    sensor_data.insert(
        "light bump front right signal",
        Value::Uint16(decode_packet_50(byte_data[65], byte_data[66])),
    );
    sensor_data.insert(
        "light bump center right signal",
        Value::Uint16(decode_packet_49(byte_data[63], byte_data[64])),
    );
    sensor_data.insert(
        "light bump center left signal",
        Value::Uint16(decode_packet_48(byte_data[61], byte_data[62])),
    );
    sensor_data.insert(
        "light bump front left signal",
        Value::Uint16(decode_packet_47(byte_data[59], byte_data[60])),
    );
    sensor_data.insert(
        "light bump left signal",
        Value::Uint16(decode_packet_46(byte_data[57], byte_data[58])),
    );
    sensor_data.insert(
        "light bumper",
        Value::HashMap(decode_packet_45(byte_data[56])),
    );
    sensor_data.insert(
        "right encoder counts",
        Value::Uint16(decode_packet_44(byte_data[54], byte_data[55])),
    );
    sensor_data.insert(
        "left encoder counts",
        Value::Uint16(decode_packet_43(byte_data[52], byte_data[53])),
    );
    sensor_data.insert(
        "requested left velocity",
        Value::Int16(decode_packet_42(byte_data[50], byte_data[51])),
    );
    sensor_data.insert(
        "requested right velocity",
        Value::Int16(decode_packet_41(byte_data[48], byte_data[49])),
    );
    sensor_data.insert(
        "requested radius",
        Value::Int16(decode_packet_40(byte_data[46], byte_data[47])),
    );
    sensor_data.insert(
        "requested velocity",
        Value::Int16(decode_packet_39(byte_data[44], byte_data[45])),
    );
    sensor_data.insert(
        "number of stream packets",
        Value::Uint8(decode_packet_38(byte_data[43])),
    );
    sensor_data.insert("song playing", Value::Bool(decode_packet_37(byte_data[42])));

    sensor_data.insert("song number", Value::Uint8(decode_packet_36(byte_data[41])));

    sensor_data.insert("io mode", Value::Uint8(decode_packet_35(byte_data[40])));

    sensor_data.insert(
        "charging sources available",
        Value::HashMap(decode_packet_34(byte_data[39])),
    );
    sensor_data.insert(
        "ignored1",
        Value::Str(decode_packet_32_and_33(
            byte_data[36],
            byte_data[37],
            byte_data[38],
        )),
    );
    sensor_data.insert(
        "cliff right signal",
        Value::Uint16(decode_packet_31(byte_data[34], byte_data[35])),
    );
    sensor_data.insert(
        "cliff front right signal",
        Value::Uint16(decode_packet_30(byte_data[32], byte_data[33])),
    );
    sensor_data.insert(
        "cliff front left signal",
        Value::Uint16(decode_packet_29(byte_data[30], byte_data[31])),
    );
    sensor_data.insert(
        "cliff left signal",
        Value::Uint16(decode_packet_28(byte_data[28], byte_data[29])),
    );
    sensor_data.insert(
        "wall signal",
        Value::Uint16(decode_packet_27(byte_data[26], byte_data[27])),
    );
    sensor_data.insert(
        "battery capacity",
        Value::Uint16(decode_packet_26(byte_data[24], byte_data[25])),
    );
    sensor_data.insert(
        "battery charge",
        Value::Uint16(decode_packet_25(byte_data[22], byte_data[23])),
    );
    sensor_data.insert("temperature", Value::Int8(decode_packet_24(byte_data[21])));

    sensor_data.insert(
        "current",
        Value::Int16(decode_packet_23(byte_data[19], byte_data[20])),
    );
    sensor_data.insert(
        "voltage",
        Value::Uint16(decode_packet_22(byte_data[17], byte_data[18])),
    );
    sensor_data.insert(
        "charging state",
        Value::Uint8(decode_packet_21(byte_data[16])),
    );
    sensor_data.insert(
        "angle",
        Value::Int16(decode_packet_20(byte_data[14], byte_data[15])),
    );
    sensor_data.insert(
        "distance",
        Value::Int16(decode_packet_19(byte_data[12], byte_data[13])),
    );
    sensor_data.insert("buttons", Value::HashMap(decode_packet_18(byte_data[11])));

    sensor_data.insert(
        "infrared char omni",
        Value::Uint8(decode_packet_17(byte_data[10])),
    );
    sensor_data.insert("ignored2", Value::Str(decode_packet_16(byte_data[9])));

    sensor_data.insert("dirt detect", Value::Str(decode_packet_15(byte_data[8])));

    sensor_data.insert(
        "wheel overcurrents",
        Value::HashMap(decode_packet_14(byte_data[7])),
    );
    sensor_data.insert("virtual wall", Value::Bool(decode_packet_13(byte_data[6])));

    sensor_data.insert("cliff right", Value::Bool(decode_packet_12(byte_data[5])));

    sensor_data.insert(
        "cliff front right",
        Value::Bool(decode_packet_11(byte_data[4])),
    );
    sensor_data.insert(
        "cliff front left",
        Value::Bool(decode_packet_10(byte_data[3])),
    );
    sensor_data.insert("cliff left", Value::Bool(decode_packet_9(byte_data[2])));

    sensor_data.insert("wall seen", Value::Bool(decode_packet_8(byte_data[1])));

    sensor_data.insert(
        "wheel drop and bumps",
        Value::HashMap(decode_packet_7(byte_data[0])),
    );
}
