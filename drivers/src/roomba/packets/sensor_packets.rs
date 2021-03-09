use crate::roomba::decode::{
    decode_packet_13, decode_packet_21, decode_packet_22, decode_packet_24, decode_packet_25,
    decode_packet_26, decode_packet_35, decode_packet_39, decode_packet_40, decode_packet_41,
    decode_packet_42, decode_packet_43, decode_packet_44, decode_packet_45, decode_packet_58,
};
use crate::utils::enums::{inspect, Value};
use std::collections::HashMap;

pub fn decode_sensor_packets(byte_data: &mut Vec<u8>) -> HashMap<&'static str, Value> {
    let mut sensor_data = HashMap::new();

    if byte_data.remove(0) == 13 {
        sensor_data.insert(
            "virtual wall",
            Value::Bool(decode_packet_13(byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 21 {
        sensor_data.insert(
            "charging state",
            Value::Uint8(decode_packet_21(byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 22 {
        sensor_data.insert(
            "voltage",
            Value::Uint16(decode_packet_22(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 24 {
        sensor_data.insert(
            "temperature",
            Value::Int8(decode_packet_24(byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 25 {
        sensor_data.insert(
            "battery charge",
            Value::Uint16(decode_packet_25(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 26 {
        sensor_data.insert(
            "battery capacity",
            Value::Uint16(decode_packet_26(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 35 {
        sensor_data.insert(
            "oi mode",
            Value::Uint8(decode_packet_35(byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 39 {
        sensor_data.insert(
            "requested velocity",
            Value::Int16(decode_packet_39(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 40 {
        sensor_data.insert(
            "requested radius",
            Value::Int16(decode_packet_40(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 41 {
        sensor_data.insert(
            "requested right velocity",
            Value::Int16(decode_packet_41(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 42 {
        sensor_data.insert(
            "requested left velocity",
            Value::Int16(decode_packet_42(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 43 {
        sensor_data.insert(
            "left encoder counts",
            Value::Uint16(decode_packet_43(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 44 {
        sensor_data.insert(
            "right encoder counts",
            Value::Uint16(decode_packet_44(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 45 {
        sensor_data.insert(
            "light bumper",
            Value::HashMap(decode_packet_45(byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 58 {
        sensor_data.insert(
            "stasis",
            Value::HashMap(decode_packet_58(byte_data.remove(0))),
        );
    }
    // for (key, value) in &sensor_data {
    //     println!("{}: {:?}", key, inspect(&value));
    // }
    sensor_data
}
