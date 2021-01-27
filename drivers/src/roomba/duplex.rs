use crate::roomba::decode::*;
use byteorder::{BigEndian, ByteOrder};
use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::io::{Cursor, Error, Read, Write};
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
    port.write(&[START]);
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

fn inspect(value: Value) -> String {
    match value {
        Value::Str(v) => {
            format!("{}", v)
        }
        Value::Bool(v) => {
            format!("{}", v)
        }
        Value::Int16(v) => {
            format!("{}", v)
        }
        Value::Uint16(v) => {
            format!("{}", v)
        }
        Value::Int8(v) => {
            format!("{}", v)
        }
        Value::Uint8(v) => {
            format!("{}", v)
        }
        Value::HashMap(v) => {
            format!("{:?}", v)
        }
    }
}

pub fn decode_sensor_packets(byte_data: [u8; 80]) {
    let mut sensor_data = HashMap::new();

    let mut vec = byte_data.to_vec();

    assert_eq!(vec.len(), 80);

    // size 80, contains 7-58 (ALL)
    sensor_data.insert(
        "stasis",
        Value::HashMap(decode_packet_58(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "side brush motor current",
        Value::Int16(decode_packet_57(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "main brush motor current",
        Value::Int16(decode_packet_56(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "right motor current",
        Value::Int16(decode_packet_55(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "left motor current",
        Value::Int16(decode_packet_54(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "infrared char right",
        Value::Uint8(decode_packet_53(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "infrared char left",
        Value::Uint8(decode_packet_52(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "light bump right signal",
        Value::Uint16(decode_packet_51(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "light bump front right signal",
        Value::Uint16(decode_packet_50(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "light bump center right signal",
        Value::Uint16(decode_packet_49(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "light bump center left signal",
        Value::Uint16(decode_packet_48(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "light bump front left signal",
        Value::Uint16(decode_packet_47(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "light bump left signal",
        Value::Uint16(decode_packet_46(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "light bumper",
        Value::HashMap(decode_packet_45(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "right encoder counts",
        Value::Uint16(decode_packet_44(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "left encoder counts",
        Value::Uint16(decode_packet_43(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "requested left velocity",
        Value::Int16(decode_packet_42(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "requested right velocity",
        Value::Int16(decode_packet_41(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "requested radius",
        Value::Int16(decode_packet_40(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "requested velocity",
        Value::Int16(decode_packet_39(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "number of stream packets",
        Value::Uint8(decode_packet_38(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "song playing",
        Value::Bool(decode_packet_37(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "song number",
        Value::Uint8(decode_packet_36(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "io mode",
        Value::Uint8(decode_packet_35(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "charging sources available",
        Value::HashMap(decode_packet_34(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "ignored1",
        Value::Str(decode_packet_32_and_33(
            vec.pop().unwrap(),
            vec.pop().unwrap(),
            vec.pop().unwrap(),
        )),
    );
    sensor_data.insert(
        "cliff right signal",
        Value::Uint16(decode_packet_31(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "cliff front right signal",
        Value::Uint16(decode_packet_30(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "cliff front left signal",
        Value::Uint16(decode_packet_29(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "cliff left signal",
        Value::Uint16(decode_packet_28(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "wall signal",
        Value::Uint16(decode_packet_27(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "battery capacity",
        Value::Uint16(decode_packet_26(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "battery charge",
        Value::Uint16(decode_packet_25(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "temperature",
        Value::Int8(decode_packet_24(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "current",
        Value::Int16(decode_packet_23(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "voltage",
        Value::Uint16(decode_packet_22(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "charging state",
        Value::Uint8(decode_packet_21(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "angle",
        Value::Int16(decode_packet_20(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "distance",
        Value::Int16(decode_packet_19(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "buttons",
        Value::HashMap(decode_packet_18(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "infrared char omni",
        Value::Uint8(decode_packet_17(vec.pop().unwrap())),
    );
    sensor_data.insert("ignored2", Value::Str(decode_packet_16(vec.pop().unwrap())));

    sensor_data.insert(
        "dirt detect",
        Value::Str(decode_packet_15(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "wheel overcurrents",
        Value::HashMap(decode_packet_14(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "virtual wall",
        Value::Bool(decode_packet_13(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "cliff right",
        Value::Bool(decode_packet_12(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "cliff front right",
        Value::Bool(decode_packet_11(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "cliff front left",
        Value::Bool(decode_packet_10(vec.pop().unwrap())),
    );
    sensor_data.insert(
        "cliff left",
        Value::Bool(decode_packet_9(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "wall seen",
        Value::Bool(decode_packet_8(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "wheel drop and bumps",
        Value::HashMap(decode_packet_7(vec.pop().unwrap())),
    );

    assert_eq!(vec.len(), 0);

    for (key, value) in sensor_data {
        println!("{}: {:?}", key, inspect(value));
    }
}

pub fn decode_battery_packets(byte_data: [u8; 10]) {
    let mut sensor_data = HashMap::new();

    let mut vec = byte_data.to_vec();

    //size 10, contains 21-26clone.flush().unwrap();
    sensor_data.insert(
        "battery capacity",
        Value::Uint16(decode_packet_26(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "battery charge",
        Value::Uint16(decode_packet_25(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "temperature",
        Value::Int8(decode_packet_24(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "current",
        Value::Int16(decode_packet_23(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "voltage",
        Value::Uint16(decode_packet_22(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "charging state",
        Value::Uint8(decode_packet_21(vec.pop().unwrap())),
    );

    for (key, value) in sensor_data {
        println!("{}: {:?}", key, inspect(value));
    }
}
pub fn decode_odom_packets(byte_data: [u8; 6]) {
    let mut sensor_data = HashMap::new();

    let mut vec = byte_data.to_vec();
    assert_eq!(vec.len(), 6);
    // size 6, contains 17-20
    sensor_data.insert(
        "angle",
        Value::Int16(decode_packet_20(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "distance",
        Value::Int16(decode_packet_19(vec.pop().unwrap(), vec.pop().unwrap())),
    );
    sensor_data.insert(
        "buttons",
        Value::HashMap(decode_packet_18(vec.pop().unwrap())),
    );

    sensor_data.insert(
        "infrared char omni",
        Value::Uint8(decode_packet_17(vec.pop().unwrap())),
    );

    assert_eq!(vec.len(), 0);

    for (key, value) in sensor_data {
        println!("{}: {:?}", key, inspect(value));
    }
}
