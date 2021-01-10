extern crate drivers;

use drivers::roomba;

#[test]
fn list_available_ports_test() {
    roomba::reading::list_ports()
}
