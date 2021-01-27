use drivers;

use drivers::roomba::read_states::sensors;
use drivers::roomba::{drive, duplex, reading};

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    duplex::duplex();
    //sensors()
    //drive::drive();
}
