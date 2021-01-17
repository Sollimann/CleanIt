extern crate drivers;

use drivers::roomba;
use drivers::roomba::decode::{decode_bool, decode_short, decode_unsigned_short};

#[test]
fn list_available_ports_test() {
    roomba::reading::list_ports()
}

#[test]
fn read_from_port() {
    roomba::reading::open_and_configure_port()
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
    let byte_array = [1];
    let value = decode_bool(byte_array[0]);
    assert_eq!(value, true);
}
