use crate::roomba::decode::{decode_packet_13, decode_packet_29};
use crate::utils::checksum::Checksum;
use crate::utils::enums::{inspect, Value};
use serialport::SerialPort;
use std::alloc::Global;
use std::collections::HashMap;
use std::time::Duration;
use std::{io, thread};

// opcode
const STREAM: u8 = 148_u8;
const NR_OF_PACKS_REQUESTED: u8 = 2_u8;
const PACKET_29_13: [u8; 4] = [STREAM, NR_OF_PACKS_REQUESTED, 29, 13];

// head and tail size
const HEADER_BYTE: u8 = 19;
const HEADER_SIZE: u8 = 1_u8;
const CHECKSUM_SIZE: u8 = 1_u8;

// packet sizes
const PACKET_29_SIZE: u8 = 2_u8;
const PACKET_13_SIZE: u8 = 1_u8;
const NBYTES: u8 = NR_OF_PACKS_REQUESTED + PACKET_29_SIZE + PACKET_13_SIZE;

const READ_BUFFER_SIZE: usize = (HEADER_SIZE + NBYTES + CHECKSUM_SIZE) as usize;

pub fn read_serial_stream(mut port: Box<dyn SerialPort, Global>) -> Box<dyn SerialPort, Global> {
    let write_buffer = PACKET_29_13;
    let mut read_buffer = [0u8; 16];
    let nbytes = NBYTES;

    // init checksum
    let mut checksum = Checksum::new();

    // Read the response from the cloned port
    port.flush().unwrap();
    port.write_all(&write_buffer)
        .expect("Failed to write to serial port");

    let mut _count = 1;
    loop {
        match port.read(&mut read_buffer) {
            Ok(bytes_recvd) => {
                _count += 1;
                println!("count: {}", _count);
                println!("buffer size: {} bytes", bytes_recvd);
                println!("buffer content: {:?}", &read_buffer);

                if bytes_recvd == read_buffer.len() {
                    checksum.push_slice(&read_buffer);
                    let mut byte_data = read_buffer.to_vec();
                    read_if_not_corrupt(&mut checksum, &mut byte_data, nbytes);
                } else {
                    println!("corrupted buffer")
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("This is an error: {:?}", e),
        };
        port.flush().unwrap();
        thread::sleep(Duration::from_millis(15));

        if _count > 300 {
            break;
        }
    }
    port
}

pub fn read_if_not_corrupt(checksum: &mut Checksum, byte_data: &mut Vec<u8, Global>, nbytes: u8) {
    let checksum_low_byte = checksum.calculate_low_byte_sum();
    let sanitize_ok = sanitize(byte_data, nbytes);

    println!("checksum low byte: {}", checksum_low_byte);

    if checksum_low_byte == 0 && sanitize_ok {
        checksum.reset();

        println!("running decode stream with byte data: {:?}", byte_data);
        decode_relevant_states(byte_data);
    } else {
        println!("checksum or/and sanitize is wrong")
    }
}

fn decode_relevant_states(byte_data: &mut Vec<u8, Global>) {
    let mut sensor_data = HashMap::new();

    if byte_data.remove(0) == 29 {
        sensor_data.insert(
            "cliff front left signal",
            Value::Uint16(decode_packet_29(byte_data.remove(1), byte_data.remove(0))),
        );
    }

    if byte_data.remove(0) == 13 {
        sensor_data.insert(
            "virtual wall",
            Value::Bool(decode_packet_13(byte_data.remove(0))),
        );
    }

    for (key, value) in sensor_data {
        println!("{}: {:?}", key, inspect(value));
    }
}

fn sanitize(byte_data: &mut Vec<u8, Global>, nbytes: u8) -> bool {
    println!("byte_data before sanitize: {:?}", byte_data);

    let header = byte_data.remove(0);
    let n = byte_data.remove(0);

    // remove header
    if header != HEADER_BYTE {
        println!("{} != header byte", header);
        return false;
    }

    // remove nbytes
    if n != nbytes {
        println!("{} != nbytes", n);
        return false;
    }

    // remove checksum
    byte_data.pop();

    println!("byte_data after sanitize: {:?}", byte_data);
    true
}
