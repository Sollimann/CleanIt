// standard libs
use std::collections::HashMap;
use std::time::Duration;
use std::{io, thread};

// internal libs
use crate::roomba::decode::{decode_packet_13, decode_packet_29};
use crate::utils::checksum::Checksum;
use crate::utils::enums::{inspect, Value};
use crate::utils::vector_manipulation::extract_sublist;

// get custom protos
use proto::roomba_service_protos as protos;
use protos::{LightBumper, SensorData, Stasis};

// custom libs
use colored::*;
use serialport::SerialPort;

// opcode
const STREAM: u8 = 148_u8;
// const NR_OF_PACKS_REQUESTED: u8 = 2_u8;
// const PACKET_29_13: [u8; 4] = [STREAM, NR_OF_PACKS_REQUESTED, 29, 13];
// const NBYTES: u8 = 5;

// head and tail size
const HEADER_BYTE: u8 = 19;

// robot sensor packages wanted
const NR_OF_SENSOR_PACKS_REQUESTED: u8 = 15_u8;
const NR_OF_SENSOR_BYTES_RECIEVED: u8 = 39_u8;
const SENSOR_BUFFER: [u8; 84] = [0u8; 84]; // size: 2 * (header + nbytes + sensor bytes)
const BUFFER_SLICE: usize = 42;
const SENSOR_PACKAGES_WANTED: [u8; 17] = [
    STREAM,
    NR_OF_SENSOR_PACKS_REQUESTED,
    13,
    21,
    22,
    24,
    25,
    26,
    35,
    39,
    40,
    41,
    42,
    43,
    44,
    45,
    58,
];

use futures_core::stream::Stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

pub fn yield_sensor_stream(
    mut port: Box<dyn SerialPort>,
    f: fn(&mut Vec<u8>) -> SensorData,
) -> impl Stream<Item = SensorData> {
    let write_buffer = SENSOR_PACKAGES_WANTED;

    // let the buffer size be twice the expected size
    let mut read_buffer = SENSOR_BUFFER;
    let nbytes = NR_OF_SENSOR_BYTES_RECIEVED;
    let slice_size = BUFFER_SLICE;

    // init checksum
    let mut checksum = Checksum::new();

    // Read the response from the cloned port
    port.flush().unwrap();
    port.write_all(&write_buffer)
        .expect("Failed to write to serial port");

    // macro
    async_stream::stream! {
        loop {
            match port.read(&mut read_buffer) {
                Ok(_) => {
                    let mut byte_data = read_buffer.to_vec();

                    if extract_sublist(&mut byte_data, [19, nbytes], slice_size, &mut checksum) {
                        match sanitize_and_read(&mut byte_data, nbytes, f) {
                            Some(sensor_readings) => yield sensor_readings,
                            None => println!("sanitizing failed")
                        }
                        port.flush().unwrap();
                        // println!("{}", "OK!".green());
                    } else {
                        port.flush().unwrap();
                        // println!("{}", "corrupted buffer".red());
                    }
                }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("This is an error: {:?}", e),
            };
            port.flush().unwrap();
            thread::sleep(Duration::from_millis(20));
        }
    }
}

pub fn sanitize_and_read(
    byte_data: &mut Vec<u8>,
    nbytes: u8,
    f: fn(&mut Vec<u8>) -> SensorData,
) -> Option<SensorData> {
    let sanitize_ok = sanitize(byte_data, nbytes);

    match sanitize_ok {
        true => Some(f(byte_data)),
        false => None,
    }
}

fn sanitize(byte_data: &mut Vec<u8>, nbytes: u8) -> bool {
    let header = byte_data.remove(0);
    let n = byte_data.remove(0);

    // remove header
    if header != HEADER_BYTE {
        return false;
    }

    // remove nbytes
    if n != nbytes {
        return false;
    }

    // remove checksum
    byte_data.pop();

    true
}
