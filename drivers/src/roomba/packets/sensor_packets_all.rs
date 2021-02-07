use crate::roomba::decode::*;
use crate::utils::enums::{inspect, Value};
use std::collections::HashMap;

pub fn decode_all_sensor_packets(byte_data: [u8; 80]) {
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

    for (key, value) in &sensor_data {
        println!("{}: {:?}", key, inspect(&value));
    }
}
