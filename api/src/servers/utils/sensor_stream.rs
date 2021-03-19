// get custom protos
use proto::roomba_service_protos as protos;
use protos::{LightBumper, SensorData, Stasis};

// drivers
use drivers::roomba::drive::drive_direct;
use drivers::roomba::packets::sensor_packets::decode_sensor_packets_as_proto;
use drivers::roomba::serial_stream::yield_sensor_stream;
use drivers::roomba::startup::{shutdown, startup};

// grpc tools
use async_std::task;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

// standard lib (threading, time, mutex, hashing)
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

async fn drive_and_sense() {
    let mut port = startup();
    let port_clone = port.try_clone().expect("Failed to clone");

    // write sensor data to a shared buffer
    // https://squidarth.com/rc/rust/2018/06/04/rust-concurrency.html
    let sensor_buffer: Arc<Mutex<Vec<SensorData>>> = Arc::new(Mutex::new(vec![]));
    let buffer_clone = sensor_buffer.clone();

    // read sensor values in one thread
    task::spawn(async move {
        //read_serial_stream(clone, decode_sensor_packets); // 50hz
        let sensor_stream = yield_sensor_stream(port_clone, decode_sensor_packets_as_proto);
        pin_mut!(sensor_stream); // needed for iteration

        while let Some(sensor_data) = sensor_stream.next().await {
            buffer_clone.lock().unwrap().push(sensor_data);
        }
    });

    thread::spawn(move || loop {
        thread::sleep(Duration::from_millis(20));
        let mut data = sensor_buffer.lock().unwrap();
        if data.len() > 0 {
            println!("data size: {}", data.len());
            data.pop();
        }
    });

    // drive the roomba_service in main thread
    //port = drive(100, 200, port);
    port = drive_direct(55, 55, port);
    thread::sleep(Duration::from_millis(5000));
    port = drive_direct(0, 0, port);
    thread::sleep(Duration::from_millis(1000));
    shutdown(port);
}
