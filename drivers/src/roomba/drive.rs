use crate::roomba::read_states::sensors;
use crate::roomba::startup::{shutdown, startup};
use std::thread;
use std::time::Duration;

pub fn drive() {
    const DOCK: u8 = 143_u8;

    let mut port = startup();

    // drive forward for 0.5 sec
    let drive = [145, 0, 50, 0, 50];
    let stop = [145, 0, 0, 0, 0];
    port.write(&drive);
    thread::sleep(Duration::from_millis(15));
    port = sensors(port);
    //thread::sleep(Duration::from_millis(6000));
    port.write(&stop);
    thread::sleep(Duration::from_millis(4000));
    //port.write(&[DOCK]);
    //thread::sleep(Duration::from_millis(2000));
    shutdown(port);
}
