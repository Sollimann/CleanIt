use crate::roomba::decode::{decode_packet_13, decode_packet_29};
use crate::utils::enums::{inspect, Value};
use std::collections::HashMap;

pub fn decode_example_packets(byte_data: &mut Vec<u8>) -> HashMap<&'static str, Value> {
    let mut sensor_data = HashMap::new();

    if byte_data.remove(0) == 29 {
        sensor_data.insert(
            "cliff front left signal",
            Value::Uint16(decode_packet_29(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 13 {
        sensor_data.insert(
            "virtual wall",
            Value::Bool(decode_packet_13(byte_data.remove(0))),
        );
    }

    for (key, value) in &sensor_data {
        println!("{}: {:?}", key, inspect(&value));
    }

    sensor_data
}
