#[derive(Debug)]
struct RoombaSensorsService;

pub mod roombasensors {
    tonic::include_proto!("roombasensors");
}

use drivers::roomba::drive;
use roombasensors::roomba_sensors_server::{RoombaSensors, RoombaSensorsServer};
use roombasensors::{LightBumper, Sensors, SensorsReceived, Stasis};

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    //duplex::duplex();
    drive::drive_and_sense();
    //mode_commands();
}
