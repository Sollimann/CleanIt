use hex::encode;
use parse_int::parse;

const HEX_PREFIX: &str = "0x";

/// Decode a 16 bit short from two bytes
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
    let one_byte_buffer = [byte];
    let encoded_hex = encode(one_byte_buffer);
    let prefixed_hex = format!("{}{}", HEX_PREFIX, encoded_hex);
    let decoded_decimal = parse::<u8>(&prefixed_hex);
    let value = decoded_decimal.unwrap() as u8;
    value != 0
}

/// Decode Packet 57 (Side Brush Motor Current) and return its value
///
/// This returns the current being drawn by the side brush motor as an unsigned 16 bit value, high byte first.
/// Range: -32768 – 32767 mA
///
/// Example:
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
/// Example:
///
/// Arguments:
///     byte: The byte to decode
///
/// Returns: True if robot is making forward progress, else False
pub fn decode_packet_58(byte: u8) -> bool {
    decode_bool(byte)
}
