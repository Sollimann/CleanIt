use std::thread;
use std::time::Duration;

pub fn drive() {
    let port_name = &serialport::available_ports().expect("No serial port")[0].port_name;
    let mut port = serialport::new(port_name, 115_200)
        .open()
        .expect("Failed to open serial port");

    // set robot in mode
    const START: u8 = 128_u8;
    const FULL: u8 = 132_u8;
    const STOP: u8 = 173_u8;
    const DOCK: u8 = 143_u8;

    // Write a buffer into this writer, returning how many bytes were written.
    // https://doc.rust-lang.org/nightly/std/io/trait.Write.html
    let mut msg = port.write(&[STOP]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));
    msg = port.write(&[FULL]);
    println!("{:?}", msg);
    thread::sleep(Duration::from_millis(15));

    // drive forward for 0.5 sec
    let drive = [145, 0, 100, 0, 100];
    let stop = [145, 0, 0, 0, 0];
    port.write(&drive);
    thread::sleep(Duration::from_millis(1000));
    port.write(&stop);
    thread::sleep(Duration::from_millis(4000));
    port.write(&[DOCK]);
    thread::sleep(Duration::from_millis(2000));
    port.write(&stop);
    port.write(&[STOP]);
}
