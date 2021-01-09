extern crate serialport;

use serialport::{available_ports, SerialPortType};

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
