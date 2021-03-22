// get custom protos
use proto::roomba_service_protos as protos;
use protos::{SensorData, SensorsReceived};

// grpc
use futures::StreamExt;
use tonic::{Request, Response, Status};

// get standard library utils
use crate::servers::endpoints::RoombaService;
use colored::Colorize;
use drivers::roomba::startup::shutdown;
use std::marker::Sync;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

impl RoombaService {
    pub async fn handle_send_sensor_stream(
        &self,
        request: Request<tonic::Streaming<SensorData>>,
    ) -> Result<Response<SensorsReceived>, Status> {
        let mut stream = request.into_inner();

        let mut received = SensorsReceived::default();

        while let Some(sensor_readings) = stream.next().await {
            match sensor_readings {
                Ok(data) => {
                    println!("  ==> Sensors = {:?}", data);
                    self.tx.send(data).unwrap();
                    //self.push_sensor_data_to_buffer(data);
                }
                Err(err) => {
                    panic!("Something went wrong unwrapping sensor readings: {:?}", err)
                }
            };

            // let rc = self.rx.recv().unwrap();
            // println!("received {:?}", rc);

            // Increment the point count
            received.status = true;
            received.packet_count += 1;
        }

        Ok(Response::new(received))
    }
}
