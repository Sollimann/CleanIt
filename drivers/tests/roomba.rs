extern crate drivers;

use drivers::roomba;
use drivers::roomba::decode::{
    decode_bool, decode_individual_bits, decode_packet_58, decode_short, decode_unsigned_short,
};

#[test]
fn list_available_ports_test() {
    roomba::reading::list_ports()
}

#[test]
fn read_from_port() {
    roomba::reading::open_and_configure_port()
}

#[test]
fn decode_individual_bits_of_byte() {
    let bits = decode_individual_bits(46);
    println!("{:?}", bits);
    assert_eq!(bits.len(), 8);
}

#[test]
fn decode_two_bytes_as_signed_16_bit() {
    let byte_array = [255, 56];
    let value = decode_short(byte_array[0], byte_array[1]);
    assert_eq!(value, -200);
}

#[test]
fn decode_two_bytes_as_unsigned_16_bit() {
    let byte_array = [255, 56];
    let value = decode_unsigned_short(byte_array[0], byte_array[1]);
    assert_eq!(value, 65336);
}

#[test]
fn decode_one_bytes_as_boolean() {
    let byte_array = [6];
    let value = decode_bool(byte_array[0]);
    assert_eq!(value, true);
}

#[test]
fn decode_statis_package_58() {
    let byte = 123;
    let hashmap = decode_packet_58(byte);
    println!("hashmap: {:?}", hashmap);
}
