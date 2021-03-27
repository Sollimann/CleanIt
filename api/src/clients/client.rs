// get custom protos
use proto::roomba_service_protos as protos;
use protos::roomba_client::RoombaClient;
use protos::{LightBumper, SensorData, SensorsReceived, SensorsRequest, Stasis};

// grpc tools
use drivers::roomba::packets::sensor_packets::decode_sensor_packets_as_proto;
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

// get standard library utils
use colored::Colorize;
use drivers::roomba::drive::drive_direct;
use drivers::roomba::startup::{shutdown, startup};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc;
use tokio::sync::mpsc::Receiver;
use tokio::time;
use tokio::time::Duration;

pub async fn client_side_stream(
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

pub async fn get_sensor_data(client: &mut RoombaClient<Channel>) -> Result<(), Box<dyn Error>> {
    let request = SensorsRequest {
        stream_frequency: 20, // Hz
    };

    let mut stream = client
        .get_sensor_data(Request::new(request))
        .await?
        .into_inner();

    let mut count: u32 = 0;
    while let Some(data) = stream.message().await? {
        thread::sleep(Duration::from_millis(20));
        println!("some data = {:?}", data);
        println!("{}", "receiving data from server".green());
        count += 1;
        println!("count: {}", &count);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client1 = RoombaClient::connect("http://[::1]:10002").await?;
    // let mut client2 = RoombaClient::connect("http://[::1]:10006").await?;
    let mut client2 = client1.clone();

    println!("\n*** CLIENT STREAMING ***");

    let mut port = startup();
    let port_clone = port.try_clone().expect("Failed to clone");

    let (tx, rx) = mpsc::channel(300);
    task::spawn(async move {
        //read_serial_stream(clone, decode_sensor_packets); // 50hz
        let sensor_stream = yield_sensor_stream(port_clone, decode_sensor_packets_as_proto);
        pin_mut!(sensor_stream); // needed for iteration

        while let Some(sensor_data) = sensor_stream.next().await {
            if tx.send(sensor_data).await.is_err() {
                eprintln!("{}", "receiver dropped!".red());
                return;
            }
        }
    });

    tokio::spawn(async move {
        match client_side_stream(&mut client1, rx).await {
            Ok(_) => println!("{}", "client side stream: OK!".green()),
            Err(e) => eprintln!("Something went wrong: {:?}", e),
        }
    });

    // give some time for service to start

    tokio::spawn(async move {
        match get_sensor_data(&mut client2).await {
            Ok(_) => println!("{}", "get sensor data: OK!".green()),
            Err(e) => eprintln!("Something went wrong: {:?}", e),
        }
    });

    // give some time for service to start
    thread::sleep(Duration::from_millis(1500));

    port = drive_direct(35, 35, port);
    thread::sleep(Duration::from_millis(5000));
    port = drive_direct(0, 0, port);
    thread::sleep(Duration::from_millis(1000));
    shutdown(port);

    Ok(())
}
