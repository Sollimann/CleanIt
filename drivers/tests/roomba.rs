extern crate drivers;

use drivers::roomba;
use drivers::roomba::decode::{
    decode_bool, decode_byte, decode_individual_bits, decode_packet_58, decode_short,
    decode_unsigned_byte, decode_unsigned_short,
};
use drivers::roomba::duplex::decode_sensor_packets;
use drivers::roomba::serial_stream::{decode_relevant_states, sanitize_and_read};
use drivers::utils::checksum::Checksum;
use drivers::utils::vector_manipulation::extract_sublist;

// Integration tests
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
    let mut byte_array = [255, 56].to_vec();

    let value: i16 = decode_short(byte_array.pop().unwrap(), byte_array.pop().unwrap());
    assert_eq!(value, -200);
}

#[test]
fn test_decode_two_bytes_as_unsigned_16_bit() {
    let mut byte_array = [255, 56].to_vec();
    let value = decode_unsigned_short(byte_array.pop().unwrap(), byte_array.pop().unwrap());
    assert_eq!(value, 65336);
}

#[test]
fn test_decode_one_bytes_as_boolean() {
    let mut byte_array = [6].to_vec();
    let value = decode_bool(byte_array.pop().unwrap());
    assert_eq!(value, true);
}
// testing

#[test]
fn test_decode_two_bytes_as_signed_16_bit_2() {
    let mut byte_array = [23, 16].to_vec();

    let value: i16 = decode_short(byte_array.pop().unwrap(), byte_array.pop().unwrap());
    assert_eq!(value, -200);
}

#[test]
fn test_decode_two_bytes_as_unsigned_16_bit_2() {
    let mut byte_array = [2, 25].to_vec();
    let value = decode_unsigned_short(byte_array.pop().unwrap(), byte_array.pop().unwrap());
    assert_eq!(value, 549);
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

    decode_sensor_packets(buffer);
}

#[test]
fn test_decode_serial_stream() {
    let buffer_output = [13, 0, 168, 19, 5, 29, 2, 25, 13, 0, 168, 19, 5, 29, 4];
    let header_byte: u8 = 19;
    let nbytes: u8 = 5;

    let mut checksum = Checksum::new();

    let mut byte_data = buffer_output.to_vec();

    let succeeded = extract_sublist(&mut byte_data, [header_byte, nbytes], 8, &mut checksum);

    assert_eq!(byte_data, vec![19, 5, 29, 2, 25, 13, 0, 168]);
    assert_eq!(true, succeeded);

    checksum.push_slice(&buffer_output);
    sanitize_and_read(&mut byte_data, nbytes, decode_relevant_states);
}
