// get custom protos
use proto::roomba_service_protos as protos;
use protos::roomba_client::RoombaClient;
use protos::{LightBumper, SensorData, SensorsReceived, SensorsRequest, Stasis};

// grpc tools
use drivers::roomba::packets::sensor_packets::decode_sensor_packets;
use drivers::roomba::serial_stream::yield_sensor_stream;

use async_std::task;
use std::thread;

use serialport::SerialPort;
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

use futures::stream;
use futures_util::pin_mut;
use futures_util::stream::StreamExt;

async fn run_sensor_stream(client: &mut RoombaClient<Channel>) -> Result<(), Box<dyn Error>> {
    let mut sensor_readings = vec![];
    for _ in 0..100 {
        sensor_readings.push(random_sensors_values())
    }

    // create the request
    let input_stream = stream::iter(sensor_readings);
    let request = Request::new(input_stream);

    match client.send_sensor_stream(request).await {
        Ok(response) => println!("RESPONSE: {:?}", response.into_inner()),
        Err(e) => println!("Something went wrong: {:?}", e),
    }

    Ok(())
}

// async fn run_stream(
//     port_clone: Box<dyn SerialPort>,
//     client: &mut RoombaClient<Channel>,
// ) -> Result<(), Box<dyn Error>> {
//     let sensor_stream = yield_sensor_stream(port_clone, decode_sensor_packets);
//     pin_mut!(sensor_stream);
//     //pin_mut!(sensor_stream);
//     let outbound = async_stream::stream! {
//         while let Some(value) = sensor_stream.next().await {
//             yield value
//         }
//     };
//
//     let response = client.send_sensor_stream(Request::new(outbound)).await?;
//     Ok(())
// }

// get standard library utils
use drivers::roomba::drive::drive_direct;
use drivers::roomba::startup::{shutdown, startup};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::time;

#[derive(Debug, Clone)]
pub struct RoombaClientStream {
    sensor_buffer: Arc<Mutex<Vec<SensorData>>>,
}
impl RoombaClientStream {
    pub fn init() -> RoombaClientStream {
        RoombaClientStream {
            sensor_buffer: Arc::new(Mutex::new(vec![])),
        }
    }
    pub fn push_sensor_data_to_buffer(&self, sensor_data: SensorData) {
        let buffer_clone = self.sensor_buffer.clone();
        buffer_clone.lock().unwrap().push(sensor_data);
    }
    fn pop_sensor_data_from_buffer(&self) -> Option<SensorData> {
        let mut sensor_buffer = self.sensor_buffer.lock().unwrap();
        if sensor_buffer.len() > 0 {
            Some(sensor_buffer.remove(0))
        } else {
            None
        }
    }
    pub async fn stream(self, client: &mut RoombaClient<Channel>) -> Result<(), Box<dyn Error>> {
        let outbound = async_stream::stream! {
            let mut interval = time::interval(Duration::from_millis(20));

            while let time = interval.tick().await {
                let data = self.pop_sensor_data_from_buffer().unwrap();
                yield data;
            }
        };
        let response = client.send_sensor_stream(Request::new(outbound)).await?;
        let mut inbound = response.into_inner();
        loop {
            println!("go")
        }
        Ok(())
    }
}

pub async fn stream(
    client: &mut RoombaClient<Channel>,
    mut rx: Receiver<SensorData>,
) -> Result<(), Box<dyn Error>> {
    let outbound = async_stream::stream! {
        while let Some(sensor_data) = rx.recv().await {
            yield sensor_data;
        }
    };

    let request = Request::new(outbound);

    match client.send_sensor_stream(request).await {
        Ok(response) => println!("RESPONSE: {:?}", response.into_inner()),
        Err(e) => println!("Something went wrong: {:?}", e),
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RoombaClient::connect("http://[::1]:10000").await?;

    println!("\n*** CLIENT STREAMING ***");

    let mut port = startup();
    let port_clone = port.try_clone().expect("Failed to clone");

    let (mut tx, rx) = mpsc::channel(100);
    task::spawn(async move {
        //read_serial_stream(clone, decode_sensor_packets); // 50hz
        let sensor_stream = yield_sensor_stream(port_clone, decode_sensor_packets);
        pin_mut!(sensor_stream); // needed for iteration

        while let Some(sensor_data) = sensor_stream.next().await {
            if let Err(_) = tx.send(sensor_data).await {
                println!("receiver dropped");
                return;
            }
        }
    });

    tokio::spawn(async move {
        match stream(&mut client, rx).await {
            Ok(_) => println!("OK!"),
            Err(e) => println!("Something went wrong: {:?}", e),
        }
    });

    port = drive_direct(55, 55, port);
    thread::sleep(Duration::from_millis(5000));
    port = drive_direct(0, 0, port);
    thread::sleep(Duration::from_millis(1000));
    shutdown(port);

    Ok(())
}

fn random_sensors_values() -> SensorData {
    let light_bumper_ex = LightBumper {
        bumper_left: false,
        bumper_front_left: true,
        bumper_center_left: true,
        bumper_center_right: false,
        bumper_front_right: false,
        bumper_right: false,
    };

    let stasis_ex = Stasis {
        toggling: 0,
        disabled: 1,
    };

    SensorData {
        virtual_wall: false,
        charging_state: 1,
        voltage: 12345,
        temperature: 18,
        battery_charge: 1000,
        battery_capacity: 2000,
        oi_mode: 3,
        requested_velocity: 50,
        requested_radius: 200,
        requested_right_velocity: 100,
        requested_left_velocity: 100,
        left_encoder_counts: 1111,
        right_encoder_counts: 1245,
        light_bumper: Some(light_bumper_ex),
        stasis: Some(stasis_ex),
    }
}
