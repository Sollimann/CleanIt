extern crate drivers;

use drivers::utils::checksum;
use drivers::utils::checksum::Checksum;

use byteorder::{BigEndian, ReadBytesExt};
use byteorder::{LittleEndian, WriteBytesExt};
use std::io::Cursor;

#[test]
fn test_big_endian_i16() {
    let mut rdr = Cursor::new(vec![255, 56]);
    // Note that we use type parameters to indicate which kind of byte order
    // we want!
    assert_eq!(-200, rdr.read_i16::<BigEndian>().unwrap());
}

#[test]
fn test_big_endian_u16() {
    let mut rdr = Cursor::new(vec![2, 25]);
    // Note that we use type parameters to indicate which kind of byte order
    // we want!
    assert_eq!(537, rdr.read_u16::<BigEndian>().unwrap());
}

#[test]
fn test_write_decimal_to_buffer() {
    let mut wtr = vec![];
    wtr.write_i16::<BigEndian>(-200).unwrap();
    assert_eq!(wtr, vec![255, 56])
}

#[test]
fn calculate_checksum_256() {
    let buffer_output: [u8; 8] = [19, 5, 29, 2, 25, 13, 0, 163];

    let mut checksum = Checksum::new();
    checksum.push_slice(&buffer_output);
    let sum = checksum.calculate();
    let low_byte_of_sum = checksum.calculate_low_byte_sum();

    assert_eq!(sum, 256);
    assert_eq!(low_byte_of_sum, 0);
}

#[test]
fn calculate_checksum_512() {
    let buffer_output: [u8; 8] = [19, 5, 29, 9, 215, 13, 0, 222];

    let mut checksum = Checksum::new();
    checksum.push_slice(&buffer_output);
    let sum = checksum.calculate();
    let low_byte_of_sum = checksum.calculate_low_byte_sum();

    assert_eq!(sum, 512);
    assert_eq!(low_byte_of_sum, 0);
}
