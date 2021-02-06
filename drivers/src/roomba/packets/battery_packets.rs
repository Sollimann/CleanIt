use crate::roomba::decode::*;
use crate::utils::enums::{inspect, Value};
use std::collections::HashMap;

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
