use async_std::task;
use drivers::roomba::drive::drive_direct;
use drivers::roomba::packets::sensor_packets::decode_sensor_packets;
use drivers::roomba::serial_stream::yield_sensor_stream;
use drivers::roomba::startup::{shutdown, startup};
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use std::thread;
use std::time::Duration;

#[tokio::main]
pub async fn drive_and_sense() {
    let mut port = startup();

    let clone = port.try_clone().expect("Failed to clone");

    // read sensor values in one thread
    task::spawn(async {
        //read_serial_stream(clone, decode_sensor_packets); // 50hz
        let sensor_reading = yield_sensor_stream(clone, decode_sensor_packets);
        pin_mut!(sensor_reading); // needed for iteration

        while let Some(value) = sensor_reading.next().await {
            println!("got {:?}", value);
        }
    });

    // drive the roomba in main thread
    //port = drive(100, 200, port);
    port = drive_direct(55, 55, port);
    thread::sleep(Duration::from_millis(5000));
    port = drive_direct(0, 0, port);
    thread::sleep(Duration::from_millis(1000));
    shutdown(port);
}

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    //duplex::duplex();
    drive_and_sense();
    //mode_commands();
}
