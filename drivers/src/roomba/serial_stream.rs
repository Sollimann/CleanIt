use crate::roomba::decode::{decode_packet_13, decode_packet_29};
use crate::utils::checksum::Checksum;
use crate::utils::enums::{inspect, Value};
use crate::utils::vector_manipulation::extract_sublist;
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
const NBYTES: u8 = 5;

pub fn read_serial_stream(
    mut port: Box<dyn SerialPort, Global>,
    f: fn(&mut Vec<u8, Global>) -> (),
) -> Box<dyn SerialPort, Global> {
    let write_buffer = PACKET_29_13;

    // let the buffer size be twice the expected size which is 8 atm
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
                let mut byte_data = read_buffer.to_vec();

                if extract_sublist(&mut byte_data, [19, 5], 8, &mut checksum) {
                    println!("Before sanitize and read: {:?}", byte_data);
                    sanitize_and_read(&mut byte_data, nbytes, f);
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

pub fn sanitize_and_read(
    byte_data: &mut Vec<u8, Global>,
    nbytes: u8,
    f: fn(&mut Vec<u8, Global>) -> (),
) {
    let sanitize_ok = sanitize(byte_data, nbytes);

    if sanitize_ok {
        println!("running decode stream with byte data: {:?}", byte_data);
        f(byte_data);
    } else {
        println!("checksum or/and sanitize is wrong")
    }
}

pub fn decode_relevant_states(byte_data: &mut Vec<u8, Global>) {
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
