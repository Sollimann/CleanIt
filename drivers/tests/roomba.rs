extern crate drivers;

use drivers::roomba;
use drivers::roomba::decode::{
    decode_bool, decode_byte, decode_individual_bits, decode_packet_58, decode_short,
    decode_unsigned_byte, decode_unsigned_short,
};
use drivers::roomba::duplex::decode_sensor_packets;

#[test]
fn list_available_ports_test() {
    roomba::reading::list_ports()
}

#[test]
fn read_from_port() {
    roomba::reading::open_and_configure_port()
}

// Test decode util functions

#[test]
fn test_decode_individual_bits_of_byte() {
    let bits = decode_individual_bits(46);
    println!("{:?}", bits);
    assert_eq!(bits.len(), 8);
}

#[test]
fn test_decode_unsigned_byte() {
    let byte: u8 = 255;
    let decoded: u8 = decode_unsigned_byte(byte);
    assert_eq!(byte, decoded)
}

#[test]
fn test_decode_byte() {
    let byte: u8 = 255;
    let decoded: i8 = decode_byte(byte);
    assert_eq!(decoded, -1);
}

#[test]
fn test_decode_two_bytes_as_signed_16_bit() {
    let byte_array = [255, 56];
    let value: i16 = decode_short(byte_array[0], byte_array[1]);
    assert_eq!(value, -200);
}

#[test]
fn test_decode_two_bytes_as_unsigned_16_bit() {
    let byte_array = [255, 56];
    let value = decode_unsigned_short(byte_array[0], byte_array[1]);
    assert_eq!(value, 65336);
}

#[test]
fn test_decode_one_bytes_as_boolean() {
    let byte_array = [6];
    let value = decode_bool(byte_array[0]);
    assert_eq!(value, true);
}

// Test decode packages

#[test]
fn test_decode_stasis_package_58() {
    let byte = 123;
    let hashmap = decode_packet_58(byte);
    println!("hashmap: {:?}", hashmap);
}

#[test]
fn test_decode_all_sensor_data() {
    let buffer: [u8; 80] = [
        2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 60, 248, 255, 145, 20, 7, 110, 8, 20, 0, 0, 9, 68, 9, 128, 9, 92, 8, 28, 0, 0, 0, 0, 1,
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3, 0, 3, 0, 0, 4, 0, 0, 0, 8, 0, 0, 0,
    ];

    decode_sensor_packets(buffer)
}
