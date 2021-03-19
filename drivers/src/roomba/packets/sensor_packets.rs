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
    for (key, value) in &sensor_data {
        println!("{}: {:?}", key, inspect(&value));
    }
    sensor_data
}

// get custom protos
use proto::roomba_service_protos as protos;
use protos::{LightBumper, SensorData, SensorsReceived, SensorsRequest, Stasis};

pub fn decode_sensor_packets_as_proto(byte_data: &mut Vec<u8>) -> SensorData {
    let mut virtual_wall: bool = false;
    let mut charging_state: u8 = 0;
    let mut voltage: u16 = 0;
    let mut temperature: i8 = 0;
    let mut battery_charge: u16 = 0;
    let mut batter_capacity: u16 = 0;
    let mut oi_mode: u8 = 0;
    let mut requested_velocity: i16 = 0;
    let mut requested_radius: i16 = 0;
    let mut requested_right_velocity: i16 = 0;
    let mut requested_left_velocity: i16 = 0;
    let mut left_encoder_counts: u16 = 0;
    let mut right_encoder_counts: u16 = 0;
    let mut light_bumper: Option<LightBumper> = None;
    let mut stasis: Option<Stasis> = None;

    if byte_data.remove(0) == 13 {
        virtual_wall = decode_packet_13(byte_data.remove(0));
    }

    if byte_data.remove(0) == 21 {
        charging_state = decode_packet_21(byte_data.remove(0));
    }

    if byte_data.remove(0) == 22 {
        voltage = decode_packet_22(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 24 {
        temperature = decode_packet_24(byte_data.remove(0));
    }

    if byte_data.remove(0) == 25 {
        battery_charge = decode_packet_25(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 26 {
        batter_capacity = decode_packet_26(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 35 {
        oi_mode = decode_packet_35(byte_data.remove(0));
    }

    if byte_data.remove(0) == 39 {
        requested_velocity = decode_packet_39(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 40 {
        requested_radius = decode_packet_40(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 41 {
        requested_right_velocity = decode_packet_41(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 42 {
        requested_left_velocity = decode_packet_42(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 43 {
        left_encoder_counts = decode_packet_43(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 44 {
        right_encoder_counts = decode_packet_44(byte_data.remove(1), byte_data.remove(0));
    }

    if byte_data.remove(0) == 45 {
        let light_bumper_hashmap = decode_packet_45(byte_data.remove(0));

        let bumper_left = light_bumper_hashmap.get("bumper_left").unwrap();
        let bumper_front_left = light_bumper_hashmap.get("bumper_front_left").unwrap();
        let bumper_center_left = light_bumper_hashmap.get("bumper_center_left").unwrap();
        let bumper_center_right = light_bumper_hashmap.get("bumper_center_right").unwrap();
        let bumper_front_right = light_bumper_hashmap.get("bumper_front_right").unwrap();
        let bumper_right = light_bumper_hashmap.get("bumper_right").unwrap();

        light_bumper = futures_util::__private::Some(LightBumper {
            bumper_left: *bumper_left > 0,
            bumper_front_left: *bumper_front_left > 0,
            bumper_center_left: *bumper_center_left > 0,
            bumper_center_right: *bumper_center_right > 0,
            bumper_front_right: *bumper_front_right > 0,
            bumper_right: *bumper_right > 0,
        })
    }

    if byte_data.remove(0) == 58 {
        let stasis_hashmap = decode_packet_58(byte_data.remove(0));

        let disabled = stasis_hashmap.get("disabled").unwrap();
        let toggling = stasis_hashmap.get("toggling").unwrap();

        stasis = futures_util::__private::Some(Stasis {
            toggling: *toggling as u32, // u32 in proto, but u8 in hashmap. Might be a bug
            disabled: *disabled as u32,
        });
    }

    SensorData {
        virtual_wall: virtual_wall,
        charging_state: (charging_state as u32),
        voltage: (voltage as u32),
        temperature: (temperature as i32),
        battery_charge: (battery_charge as u32),
        battery_capacity: (batter_capacity as u32),
        oi_mode: (oi_mode as u32),
        requested_velocity: (requested_velocity as i32),
        requested_radius: (requested_radius as i32),
        requested_right_velocity: (requested_right_velocity as i32),
        requested_left_velocity: (requested_left_velocity as i32),
        left_encoder_counts: (left_encoder_counts as u32),
        right_encoder_counts: (right_encoder_counts as u32),
        light_bumper: light_bumper,
        stasis: stasis,
    }
}
