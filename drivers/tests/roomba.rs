extern crate drivers;

use drivers::roomba;
use drivers::roomba::decode::decode_short;

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
