extern crate serialport;

use self::serialport::SerialPort;
use serialport::{available_ports, SerialPortType};
use std::alloc::Global;
use std::io;
use std::io::Write;
use std::time::Duration;

pub fn list_ports() {
    match available_ports() {
        Ok(ports) => {
            let hits = ports.len();
            match hits {
                0 => println!("No ports found"),
                1 => println!("Found 1 port: "),
                n => println!("Found {} ports: ", n),
            };
            for p in ports {
                println!(" {}", p.port_name);
                match p.port_type {
                    SerialPortType::UsbPort(info) => {
                        println!("    Type: USB");
                        println!("    VID:{:04x} PID:{:04x}", info.vid, info.pid);
                        println!(
                            "     Serial Number: {}",
                            info.serial_number.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "      Manufacturer: {}",
                            info.manufacturer.as_ref().map_or("", String::as_str)
                        );
                        println!(
                            "           Product: {}",
                            info.product.as_ref().map_or("", String::as_str)
                        );
                    }
                    SerialPortType::BluetoothPort => {
                        println!("    Type: Bluetooth");
                    }
                    SerialPortType::PciPort => {
                        println!("    Type: PCI");
                    }
                    SerialPortType::Unknown => {
                        println!("    Type: Unknown");
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("{:?}", e);
            eprintln!("Error listing serial ports");
        }
    }
}

pub fn open_and_configure_port() {
    let port_name = "/dev/ttyUSB0";
    let baud_rate = 115_200;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(mut port) => {
            // we have to read into a buffer
            let mut serial_buf = [0; 80];

            println!(
                "Receving data on port {} at {} baud.",
                &port_name, &baud_rate
            );
            loop {
                match port.read(&mut serial_buf) {
                    Ok(t) => {
                        println!("buffer size: {} bytes", t);
                        println!("buffer content: {:?}", &serial_buf);
                        let bytes_to_string = std::str::from_utf8(&serial_buf);
                        println!("buffer as string: {:?}", bytes_to_string);
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => println!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}
