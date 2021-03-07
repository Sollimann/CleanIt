use std::collections::HashMap;
use std::marker::Sync;
use std::pin::Pin;
use std::sync::Arc;
use std::time::Instant;

//We would use tokio::sync::mpsc for communicating between futures
use tokio::sync::mpsc;

// gRPC tools
use futures::{Stream, StreamExt};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

// our messages and services
pub mod roombasensors {
    tonic::include_proto!("roombasensors");
}
use drivers::roomba::startup::startup;
use drivers::utils::enums::Value;
use roombasensors::roomba_sensors_server::{RoombaSensors, RoombaSensorsServer};
use roombasensors::{LightBumper, SensorRequest, Sensors, SensorsReceived, Stasis};
use serialport::SerialPort;

// defining a struct for our service
struct ConfigurePort {
    port: Box<dyn SerialPort>,
}

// This was done simply to please the compiler, investigate thread safety further
// https://doc.rust-lang.org/nomicon/send-and-sync.html
unsafe impl Sync for ConfigurePort {}

impl ConfigurePort {
    pub fn init() -> ConfigurePort {
        let roomba_port = startup();

        ConfigurePort { port: roomba_port }
    }

    // pass reference to port
    pub fn get_configured_port(&self) -> &Box<dyn SerialPort> {
        &self.port
    }

    // pass ownership of a clone
    pub fn get_configured_clone(&self) -> Box<dyn SerialPort> {
        self.port.try_clone().expect("Failed to clone")
    }
}

pub struct RoombaSensorsService {
    roomba_port: ConfigurePort,
}

// implementing rpc for service defined in .proto
#[tonic::async_trait]
impl RoombaSensors for RoombaSensorsService {
    async fn send_sensor_stream(
        &self,
        request: Request<tonic::Streaming<Sensors>>,
    ) -> Result<Response<SensorsReceived>, Status> {
        let mut stream = request.into_inner();

        let mut received = SensorsReceived::default();

        while let Some(sensors) = stream.next().await {
            let sensors = sensors?;

            println!("  ==> Sensors = {:?}", sensors);

            // Increment the point count
            received.status = true;
            received.packet_count += 1;
        }

        Ok(Response::new(received))
    }

    // define type alias
    #[rustfmt::skip]
    type GetSensorDataStream = Pin<Box<dyn Stream<Item = Result<Sensors, Status>>
        + Send
        + Sync
        +'static >>;

    async fn get_sensor_data(
        &self,
        request: Request<SensorRequest>,
    ) -> Result<Response<Self::GetSensorDataStream>, Status> {
        //     let sensor_reading = yield_sensor_stream(
        //         self.roomba_port.get_configured_clone(),
        //         decode_sensor_packets,
        //     );
        //     pin_mut!(sensor_reading); // needed for iteration
        //                               // output stream
        //     let output = async_stream::try_stream! {
        //         //read_serial_stream(clone, decode_sensor_packets); // 50hz
        //         //let mut data_buffer: Vec<Sensors> = Vec::new();
        //
        //         while let Some(value) = sensor_reading.next().await {
        //             //println!("got {:?}", value);
        //             let sensor_data = hashmap_to_sensor_data(value);
        //             yield sensor_data.clone();
        //             //data_buffer.push(sensor_data);
        //         }
        //     };
        //
        //     Ok(Response::new(Box::pin(output) as Self::GetSensorDataStream))
        unimplemented!("todo")
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // defining address for our service
    let addr = "[::1]:10000".parse().unwrap();

    // configure roomba connection
    let roomba_ports = ConfigurePort::init();

    // creating a service
    let sensors_service = RoombaSensorsService {
        roomba_port: roomba_ports,
    };

    println!("Server listening on {}", addr);

    let svc = RoombaSensorsServer::new(sensors_service);

    // adding our service to our server.
    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
