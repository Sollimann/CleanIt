use byteorder::{BigEndian, ByteOrder};
use drivers;

use drivers::roomba::{drive, duplex, reading};

use hex::{FromHex, ToHex};
use parse_int::parse;

use std::{fmt::Write, num::ParseIntError};

pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

pub fn encode_hex(bytes: &[u8]) -> String {
    let mut s = String::with_capacity(bytes.len() * 2);
    for &b in bytes {
        write!(&mut s, "{:02x}", b);
    }
    s
}

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    //duplex::duplex();
    //drive::drive();
    let buf = [15, 0, 0, 128, 16, 39, 240, 216, 241, 255, 127];
    let short_buf = [255, 56];

    let input = "ff38";
    let decoded = hex::decode(input).expect("Decoding failed");
    println!("decoded: {:?}", decoded);

    // encoded
    let encoded = hex::encode(decoded);
    println!("encoded: {:?}", encoded);

    let d = parse::<u16>("0xff38");

    println!("decoded as val: {:?}", d);

    //
    // println!("encode_hex: {:?}", encode_hex(&short_buf));
    // let z = i16::from_str_radix(&*encode_hex(&short_buf), 16);
    // println!("value: {:?}", z);
}
