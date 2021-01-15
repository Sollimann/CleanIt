use hex::encode;
use parse_int::parse;

const HEX_PREFIX: &str = "0x";

/// Decode a 16 bit short from two bytes
///
/// Example: (high: 255, low: 56) -> [255] [56] -> 0xFF38 = -200
///
/// Arguments:
///     low: The low byte of the 2's complement. This is specified first to make it
///          easier when popping
///     high: The high byte of the 2's complement
/// Returns: 16-bit signed value using two’s complement
pub fn decode_short(high: u8, low: u8) -> i16 {
    let two_byte_buffer = [high, low];
    let encoded_hex = encode(two_byte_buffer);
    let prefixed_hex = format!("{}{}", HEX_PREFIX, encoded_hex);
    let decoded_decimal = parse::<u16>(&prefixed_hex);
    decoded_decimal.unwrap() as i16
}
