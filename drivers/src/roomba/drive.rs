use crate::roomba::read_states::read_all_sensors;
use crate::roomba::startup::{shutdown, startup};
use std::thread;
use std::time::Duration;

pub fn drive() {
    const DOCK: u8 = 143_u8;

    let mut port = startup();

    // drive forward for 0.5 sec
    let drive = [137, 255, 56, 1, 244];
    let drive_direct = [145, 0, 50, 0, 50];
    let stop = [145, 0, 0, 0, 0];
    port.write(&drive_direct);
    thread::sleep(Duration::from_millis(15));
    port = read_all_sensors(port);
    //thread::sleep(Duration::from_millis(6000));
    port.write(&stop);
    thread::sleep(Duration::from_millis(4000));
    //port.write(&[DOCK]);
    //thread::sleep(Duration::from_millis(2000));
    shutdown(port);
}
