use hex::{decode, encode};
use parse_int::parse;
use std::borrow::Borrow;
use std::collections::hash_map::RandomState;
use std::collections::HashMap;

const HEX_PREFIX: &str = "0x";

/// gets the bit at position `n`. Bits are numbered from 0 (least significant) to 7 (most significant).
///
/// Example: get_bit_at(46, 1).unwrap() -> 2
///
/// Arguments:
///     input_byte: unsigned 8 bit byte to be decoded
///     bit_pos: bit position to be read
///
/// Returns: A specific bit value of a byte
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
pub fn decode_individual_bits<'a>(byte: u8) -> HashMap<&'a str, u8, RandomState> {
    let mut bits = HashMap::new();
    let bit0: u8 = get_bit_at(byte, 0_u8).unwrap();
    bits.insert("bit0", bit0);
    let bit1: u8 = get_bit_at(byte, 1_u8).unwrap();
    bits.insert("bit1", bit1);
    let bit2: u8 = get_bit_at(byte, 2_u8).unwrap();
    bits.insert("bit2", bit2);
    let bit3: u8 = get_bit_at(byte, 3_u8).unwrap();
    bits.insert("bit3", bit3);
    let bit4: u8 = get_bit_at(byte, 4_u8).unwrap();
    bits.insert("bit4", bit4);
    let bit5: u8 = get_bit_at(byte, 5_u8).unwrap();
    bits.insert("bit5", bit5);
    let bit6: u8 = get_bit_at(byte, 6_u8).unwrap();
    bits.insert("bit6", bit6);
    let bit7: u8 = get_bit_at(byte, 7_u8).unwrap();
    bits.insert("bit7", bit7);
    bits
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
    byte != 0
}

/// Decode Packet 33 (Unused) and return its value
///
/// The current OI mode is returned. See table below.
/// Range: 0-3
///
/// Arguments:
///     low: The first byte to be ignored
///     mid: The second byte to be ignored
///     high: The third byte to be ignored
///
/// Returns: A unsigned byte, the current OI mode id from 0-3
pub fn decode_packet_32_and_33(low: u8, mid: u8, high: u8) -> String {
    format!(
        "ignoring 3 consecutive bytes: {low}, {mid}, {high}",
        low = low,
        mid = mid,
        high = high
    )
}

/// Decode Packet 34 (charging sources available) and return its value
///
/// Roomba’s connection to the Home Base and Internal Charger are returned as individual bits, as below.
/// Range: 0-3
///
/// Arguments:
///     byte: The byte to decode
///
/// Returns: HashMap of "home base" and "internal charger"
///         1 = charging source present and powered; 0 = charging source not present or not powered.
pub fn decode_packet_34(byte: u8) -> HashMap<String, u8, RandomState> {
    let mut charging_sources_available: HashMap<String, u8> = HashMap::new();
    let bits = decode_individual_bits(byte);
    let home_base = bits.get("bit1").unwrap();
    let internal_charger = bits.get("bit0").unwrap();
    charging_sources_available.insert("home".to_string(), *home_base);
    charging_sources_available.insert("internal".to_string(), *internal_charger);
    charging_sources_available
}

/// Decode Packet 35 (OI Mode) and return its value
///
/// The current OI mode is returned. See table below.
/// Range: 0-3
///
/// Arguments:
///     byte: The byte to decode
///
/// Returns: A unsigned byte, the current OI mode id from 0-3
pub fn decode_packet_35(byte: u8) -> u8 {
    decode_unsigned_byte(byte)
}

/// Decode Packet 36 (Song number) and return its value
///
/// The currently selected OI song is returned.
/// Range: 0-15
///
/// Arguments:
///     byte: The byte to decode
///
/// Returns: unsigned byte, the current song id playing (0-15)
pub fn decode_packet_36(byte: u8) -> u8 {
    decode_unsigned_byte(byte)
}

/// Decode Packet 37 (Song playing) and return its value
///
/// The state of the OI song player is returned. 1 = OI song currently playing; 0 = OI song not playing.
/// Range: 0-1
///
/// Arguments:
///     byte: The byte to decode
///
/// Returns: True or False, stating whether the song is playing
pub fn decode_packet_37(byte: u8) -> bool {
    decode_bool(byte)
}

/// Decode Packet 38 (Number of stream packets) and return its value
///
/// The number of data stream packets is returned.
/// Range: 0-108
///
/// Arguments:
///     byte: The byte to decode
///
/// Returns: unsigned 8bit short; the number of data stream packets
pub fn decode_packet_38(byte: u8) -> u8 {
    decode_unsigned_byte(byte)
}

/// Decode Packet 39 (requested velocity) and return its value
///
/// The velocity most recently requested with a Drive command is returned as a signed 16-bit number,
/// high byte first.
/// Range: -500 - 500 mm/s
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: signed 16bit short. Velocity most recently requested by Drive()
pub fn decode_packet_39(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 40 (requested radius) and return its value
///
/// The radius most recently requested with a Drive command is returned as a signed 16-bit number, high
/// byte first. The radius is measured from the center of the turning circle to the center of Roomba.
/// A Drive command with a positive velocity and a positive radius makes Roomba drive forward while
/// turning toward the left. A negative radius makes Roomba turn toward the right.
/// Range: -32768 - 32767 mm
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// NOTE: Create 2 and Roomba 500/600 firmware versions prior to 3.3.0 return an incorrect value for
/// sensors measured in millimeters. To determine the firmware version on your robot, send a 7 via the serial
/// port to reset it. The robot will print a long welcome message which will include the firmware version, for
/// example: r3_robot/tags/release-3.3.0.
///
/// Returns: signed 16bit short. Radius most recently requested by Drive()
pub fn decode_packet_40(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 41 (Requested right velocity) and return its value
///
/// The right wheel velocity most recently requested with a Drive Direct command is returned as a signed 16-
/// bit number, high byte first.
/// Range: -500 - 500 mm/s
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: signed 16bit short. right wheel velocity recently requested by DriveDirect()
pub fn decode_packet_41(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 42 (Requested left velocity) and return its value
///
/// The left wheel velocity most recently requested with a Drive Direct command is returned as a signed 16-
/// bit number, high byte first.
/// Range: -500 - 500 mm/s
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// Returns: signed 16bit short. Left wheel velocity recently requested by DriveDirect()
pub fn decode_packet_42(high: u8, low: u8) -> i16 {
    decode_short(high, low)
}

/// Decode Packet 43 (Left Encoder Counts) and return its value
///
/// The cumulative number of raw left encoder counts is returned as a signed 16-bit number, high byte first.
/// This number will roll over if it pas    ses the max value (at approx. 14.5 meters).
/// Range: -32768 - 32767 counts
///
/// To convert counts to distance, simply do a unit conversion using the equation for circle circumference.
/// N counts * (mm in 1 wheel revolution / counts in 1 wheel revolution) = mm
/// N counts * (π * 72.0 / 508.8) = mm
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// NOTE! This actually returns a signed 16 bit in range -32768 - 32767 counts, but for ease of use
///       it is better to convert it to a unsigned 16 bit in range 0 - 65535 counts.
///
/// Returns: unsigned 16bit short. Cumulative number of raw right encoder counts. Rolls over
//           to 0 after it passes 65535
pub fn decode_packet_43(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 44 (Right Encoder Counts) and return its value
///
/// The cumulative number of raw right encoder counts is returned as a signed 16-bit number, high byte
/// first. This number will roll over if it passes the max value (at approx. 14.5 meters).
/// Range: -32768 - 32767 counts
///
/// To convert counts to distance, simply do a unit conversion using the equation for circle circumference.
/// N counts * (mm in 1 wheel revolution / counts in 1 wheel revolution) = mm
/// N counts * (π * 72.0 / 508.8) = mm
///
/// Arguments:
///     high: The high byte of the 2's complement
///     low: The low byte of the 2's complement
///
/// NOTE! This actually returns a signed 16 bit in range -32768 - 32767 counts, but for ease of use
///       it is better to convert it to a unsigned 16 bit in range 0 - 65535 counts.
///
/// Returns: unsigned 16bit short. Cumulative number of raw right encoder counts. Rolls over
///          to 0 after it passes 65535
pub fn decode_packet_44(high: u8, low: u8) -> u16 {
    decode_unsigned_short(high, low)
}

/// Decode Packet 45 (infrared char left) and return its value
///
/// The light bumper detections are returned as individual bits.
///
/// Arguments:
///     byte: The bytes to decode
///
/// Returns: A HashMapof 'light bumper'
pub fn decode_packet_45(byte: u8) -> HashMap<&'static str, u8, RandomState> {
    decode_individual_bits(byte)
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
/// Returns: HashMap of "stasis disabled" and "stasis toggling"
pub fn decode_packet_58(byte: u8) -> HashMap<String, u8, RandomState> {
    let mut stasis: HashMap<String, u8> = HashMap::new();
    let bits = decode_individual_bits(byte);
    let disabled = bits.get("bit1").unwrap();
    let toggling = bits.get("bit0").unwrap();
    stasis.insert("disabled".to_string(), *disabled);
    stasis.insert("toggling".to_string(), *toggling);
    stasis
}
