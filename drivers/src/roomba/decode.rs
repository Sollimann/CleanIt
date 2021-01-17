use bitreader::BitReader;
use hex::encode;
use parse_int::parse;
use std::collections::HashMap;

const HEX_PREFIX: &str = "0x";

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 31 (most significant).
fn get_bit_at(input_byte: u8, bit_pos: u8) -> Result<u8, String> {
    if bit_pos < 8 {
        Ok(input_byte & (1 << bit_pos))
    } else {
        let msg = format!(
            "bit position {} not valid. Valid range is 0 << bit_pos << 7",
            bit_pos
        );
        Err(msg)
    }
}

/// Read specific bits from a byte
///
/// Example:
///
/// Arguments:
///     byte: The byte to be decoded
///
/// Returns: A dict
pub fn decode_individual_bits(byte: u8) {
    let bit0: u8 = get_bit_at(byte, 0_u8).unwrap();
    let bit1: u8 = get_bit_at(byte, 1_u8).unwrap();
    let bit2: u8 = get_bit_at(byte, 2_u8).unwrap();
    let bit3: u8 = get_bit_at(byte, 3_u8).unwrap();
    let bit4: u8 = get_bit_at(byte, 4_u8).unwrap();
    let bit5: u8 = get_bit_at(byte, 5_u8).unwrap();
    let bit6: u8 = get_bit_at(byte, 6_u8).unwrap();
    let bit7: u8 = get_bit_at(byte, 8_u8).unwrap();
    let mut bits = HashMap::new();

    println!(
        "bit0: {}, bit1: {}, bit2: {}, bit3: {}, bit4: {}, bit5: {}, bit6: {}, bit7: {}",
        bit0, bit1, bit2, bit3, bit4, bit5, bit6, bit7
    );

    bits.insert("bit0", bit0);
    bits.insert("bit1", bit1);
    bits.insert("bit2", bit2);
    bits.insert("bit3", bit3);
    bits.insert("bit4", bit4);
    bits.insert("bit5", bit5);
    bits.insert("bit6", bit6);
    bits.insert("bit7", bit7);
    println!("bits: {:?}", bits);
}

/// Decode an unsigned byte. Basically return the input
///
/// Example:
///
/// Arguments:
///     byte: The byte to be decoded
///
/// Returns: An unsigned int in range 0-255
pub fn decode_unsigned_byte(byte: u8) -> u8 {
    byte
}

/// Decode a unsigned 16 bit short from two bytes
///
/// A 16-bit integer can store 2^16 (or 65,536) distinct values. In an unsigned representation,
/// these values are the integers between 0 and 65,535; using two's complement, possible values
/// range from −32,768 to 32,767. Hence, a processor with 16-bit memory addresses can directly
/// access 64 KB of byte-addressable memory.
///
/// Example: (high: 255, low: 56) -> [255] [56] -> 0xFF38 = 65336
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement. This is specified first to make it
///          easier when popping
///
/// Returns: 16-bit signed value using two’s complement
pub fn decode_unsigned_short(high: u8, low: u8) -> u16 {
    let two_byte_buffer = [high, low];
    let encoded_hex = encode(two_byte_buffer);
    let prefixed_hex = format!("{}{}", HEX_PREFIX, encoded_hex);
    parse::<u16>(&prefixed_hex).unwrap()
}

/// Decode a signed 16 bit short from two bytes
///
/// A 16-bit integer can store 2^16 (or 65,536) distinct values. In an unsigned representation,
/// these values are the integers between 0 and 65,535; using two's complement, possible values
/// range from −32,768 to 32,767. Hence, a processor with 16-bit memory addresses can directly
/// access 64 KB of byte-addressable memory.
///
/// Example: (high: 255, low: 56) -> [255] [56] -> 0xFF38 = -200
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement. This is specified first to make it
///          easier when popping
///
/// Returns: 16-bit signed value using two’s complement
pub fn decode_short(high: u8, low: u8) -> i16 {
    let two_byte_buffer = [high, low];
    let encoded_hex = encode(two_byte_buffer);
    let prefixed_hex = format!("{}{}", HEX_PREFIX, encoded_hex);
    let decoded_decimal = parse::<u16>(&prefixed_hex);
    println!("decoded dec: {:?}", decoded_decimal);
    decoded_decimal.unwrap() as i16
}

/// Decode a byte and return the value
///
/// Example: (data: 1) -> [1] -> 0x01 = 1
///
/// Arguments:
///     byte: The byte to be decoded
///
/// Returns: True or False
pub fn decode_bool(byte: u8) -> bool {
    // let one_byte_buffer = [byte];
    // let encoded_hex = encode(one_byte_buffer);
    // let prefixed_hex = format!("{}{}", HEX_PREFIX, encoded_hex);
    // let decoded_decimal = parse::<u8>(&prefixed_hex);
    // let value = decoded_decimal.unwrap() as u8;
    byte != 0
}

/// Decode Packet 46 (Light Bump Left Signal) and return its value
///
/// The strength of the light bump left signal is returned as an unsigned 16-bit value, high byte first.
/// Range: 0-4095
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: unsigned 16bit short. Strength of light bump right signal from 0-4095
pub fn decode_packet_46(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 47 (Light Bump Front Left Signal) and return its value
///
/// The strength of the light bump front left signal is returned as an unsigned 16-bit value, high byte first.
/// Range: 0-4095
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: unsigned 16bit short. Strength of light bump right signal from 0-4095
pub fn decode_packet_47(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 48 (Light Bump Center Left Signal) and return its value
///
/// The strength of the light bump center left signal is returned as an unsigned 16-bit value, high byte first.
/// Range: 0-4095
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: unsigned 16bit short. Strength of light bump right signal from 0-4095
pub fn decode_packet_48(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 49 (Light Bump Center Right Signal) and return its value
///
/// The strength of the light bump center right signal is returned as an unsigned 16-bit value, high byte first.
/// Range: 0-4095
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: unsigned 16bit short. Strength of light bump right signal from 0-4095
pub fn decode_packet_49(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 50 (Light Bump Front Right Signal) and return its value
///
/// The strength of the light bump front right signal is returned as an unsigned 16-bit value, high byte first.
/// Range: 0-4095
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: unsigned 16bit short. Strength of light bump right signal from 0-4095
pub fn decode_packet_50(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 51 (Light Bump Right Signal) and return its value
///
/// The strength of the light bump right signal is returned as an unsigned 16-bit value, high byte first.
/// Range: 0-4095
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: unsigned 16bit short. Strength of light bump right signal from 0-4095
pub fn decode_packet_51(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 52 (infrared char left) and return its value
///
/// This value identifies the 8-bit IR character currently being received by Roomba’s right receiver. A value of
/// 0 indicates that no character is being received. These characters include those sent by the Roomba
/// Remote, Dock, Virtual Walls, Create robots using the Send-IR command, and user-created devices.
/// Range: 0 – 255
///
/// Arguments:
///     data: The bytes to decode
///
/// Returns: unsigned Byte (0-255)
pub fn decode_packet_52(byte: u8) -> u8 {
    decode_unsigned_byte(byte)
}

/// Decode Packet 53 (infrared char right) and return its value
///
/// This value identifies the 8-bit IR character currently being received by Roomba’s right receiver. A value of
/// 0 indicates that no character is being received. These characters include those sent by the Roomba
/// Remote, Dock, Virtual Walls, Create robots using the Send-IR command, and user-created devices.
/// Range: 0 – 255
///
/// Arguments:
///     data: The bytes to decode
///
/// Returns: unsigned Byte (0-255)
pub fn decode_packet_53(byte: u8) -> u8 {
    decode_unsigned_byte(byte)
}

/// Decode Packet 54 (Left Motor Current) and return its value
///
/// This returns the current being drawn by the side brush motor as an unsigned 16 bit value, high byte first.
/// Range: -32768 – 32767 mA
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: signed 16bit short. Strength of side brush motor current from -32768 - 32767 mA
pub fn decode_packet_54(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 55 (Right Motor Current) and return its value
///
/// This returns the current being drawn by the side brush motor as an unsigned 16 bit value, high byte first.
/// Range: -32768 – 32767 mA
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: signed 16bit short. Strength of side brush motor current from -32768 - 32767 mA
pub fn decode_packet_55(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 56 (Main Brush Motor Current) and return its value
///
/// This returns the current being drawn by the side brush motor as an unsigned 16 bit value, high byte first.
/// Range: -32768 – 32767 mA
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: signed 16bit short. Strength of side brush motor current from -32768 - 32767 mA
pub fn decode_packet_56(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 57 (Side Brush Motor Current) and return its value
///
/// This returns the current being drawn by the side brush motor as an unsigned 16 bit value, high byte first.
/// Range: -32768 – 32767 mA
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: signed 16bit short. Strength of side brush motor current from -32768 - 32767 mA
pub fn decode_packet_57(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 58 (Stasis) and return its value
///
/// The stasis caster sensor returns 1 when the robot is making forward progress and 0 when it is not. It
/// always returns 0 when the robot is turning, driving backward, or not driving. If the stasis wheel is too
/// dirty to be read, a value of 2 is returned. If this happens, remove the stasis wheel and clean it with a
/// damp cloth, then dry it thoroughly before reinstalling the wheel.
/// Range: 0 – 3
///
/// Arguments:
///     byte: The byte to decode
///
/// Returns: True if robot is making forward progress, else False
pub fn decode_packet_58(byte: u8) -> bool {
    decode_bool(byte)
}
