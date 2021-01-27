use crate::roomba::duplex::{decode_battery_packets, decode_odom_packets, decode_sensor_packets};
use serialport::SerialPort;
use std::alloc::Global;
use std::time::Duration;
use std::{io, thread};

pub fn read_all_sensors(mut port: Box<dyn SerialPort, Global>) -> Box<dyn SerialPort, Global> {
    // Read the response from the cloned port
    let mut buffer = [0u8; 80];
    let mut _count = 1;
    loop {
        port.write_all(&[142, 100])
            .expect("Failed to write to serial port");
        port.flush().unwrap();
        match port.read(&mut buffer) {
            Ok(bytes_recvd) => {
                _count += 1;
                println!("count: {}", _count);
                println!("buffer size: {} bytes", bytes_recvd);
                println!("buffer content: {:?}", &buffer);
                if bytes_recvd == buffer.len() {
                    decode_sensor_packets(buffer);
                    //decode_battery_packets(buffer)
                    //decode_odom_packets(buffer);
                }
            }
            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
            Err(e) => eprintln!("This is an error: {:?}", e),
        };
        port.flush().unwrap();
        thread::sleep(Duration::from_millis(15));
        if _count > 500 {
            break;
        }
    }
    return port;
}
