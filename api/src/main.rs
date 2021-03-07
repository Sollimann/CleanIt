use async_std::task;
use drivers::roomba::drive::drive_direct;
use drivers::roomba::packets::sensor_packets::decode_sensor_packets;
use drivers::roomba::serial_stream::yield_sensor_stream;
use drivers::roomba::startup::{shutdown, startup};
use drivers::utils::enums::Value;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use tokio::time;
//use roomba_service::SensorData;
//use roombasensors::{LightBumper, SensorRequest, Sensors, SensorsReceived, Stasis};
use std::collections::HashMap;

#[tokio::main]
async fn drive_and_sense() {
    let mut port = startup();
    // let port_clone = port.try_clone().expect("Failed to clone");
    //
    // // write sensor data to a shared buffer
    // // https://squidarth.com/rc/rust/2018/06/04/rust-concurrency.html
    // let sensor_buffer: Arc<Mutex<Vec<Sensors>>> = Arc::new(Mutex::new(vec![]));
    // let buffer_clone = sensor_buffer.clone();
    //
    // // read sensor values in one thread
    // task::spawn(async move {
    //     //read_serial_stream(clone, decode_sensor_packets); // 50hz
    //     let sensor_stream = yield_sensor_stream(port_clone, decode_sensor_packets);
    //     pin_mut!(sensor_stream); // needed for iteration
    //
    //     while let Some(value) = sensor_stream.next().await {
    //         //println!("got {:?}", value);
    //         let sensor_data = hashmap_to_sensor_data(value);
    //         buffer_clone.lock().unwrap().push(sensor_data);
    //     }
    // });
    //
    // thread::spawn(move || loop {
    //     thread::sleep(Duration::from_millis(20));
    //     let mut data = sensor_buffer.lock().unwrap();
    //     if data.len() > 0 {
    //         println!("data size: {}", data.len());
    //         data.pop();
    //     }
    // });
    //
    // // drive the roomba_service in main thread
    // //port = drive(100, 200, port);
    // port = drive_direct(55, 55, port);
    // thread::sleep(Duration::from_millis(5000));
    // port = drive_direct(0, 0, port);
    // thread::sleep(Duration::from_millis(1000));
    // shutdown(port);
}

fn main() {
    //reading::open_and_configure_port();
    //reading::list_ports();
    //duplex::duplex();
    drive_and_sense();
    //mode_commands();
}

// fn hashmap_to_sensor_data(hashmap: HashMap<&str, Value>) -> SensorData {
//     let light_bumper_ex = LightBumper {
//         bumper_left: false,
//         bumper_front_left: true,
//         bumper_center_left: true,
//         bumper_center_right: false,
//         bumper_front_right: false,
//         bumper_right: false,
//     };
//
//     let stasis_ex = Stasis {
//         toggling: 0,
//         disabled: 1,
//     };
//
//     SensorData {
//         virtual_wall: false,
//         charging_state: 1,
//         voltage: 12345,
//         temperature: 18,
//         battery_charge: 1000,
//         battery_capacity: 2000,
//         oi_mode: 3,
//         requested_velocity: 50,
//         requested_radius: 200,
//         requested_right_velocity: 100,
//         requested_left_velocity: 100,
//         left_encoder_counts: 1111,
//         right_encoder_counts: 1245,
//         light_bumper: Some(light_bumper_ex),
//         stasis: Some(stasis_ex),
//     }
// }
