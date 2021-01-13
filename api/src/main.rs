use byteorder::{BigEndian, ByteOrder};
use drivers;

use drivers::roomba::{drive, duplex, reading};

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    //duplex::duplex();
    drive::drive();
    // let buf = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    // let value = byteorder::BigEndian::read_i16(&buf);
    // println!("{:?}", value);
    // println!("array {:?}", &buf[0..4])
    //let a = ((buf[1] as u16) << 8) | buf[0] as u16;
    //println!("{:?}", a);
}
