use drivers;

use drivers::roomba::mode::mode_commands;
use drivers::roomba::{drive, duplex, mode, reading};

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    //duplex::duplex();
    //drive::drive();
    mode_commands();
}
