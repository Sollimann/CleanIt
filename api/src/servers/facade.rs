// get custom protos
use proto::roomba_service_protos as protos;
use protos::roomba_server::{Roomba, RoombaServer};
use protos::{LightBumper, SensorData, SensorsReceived, SensorsRequest, Stasis};

// get standard library utils
use std::marker::Sync;
use std::pin::Pin;
use std::sync::{Arc, Mutex};

// gRPC tools
use futures::{Stream, StreamExt};
use tonic::{Request, Response, Status};

#[derive(Debug)]
pub struct RoombaService {
    sensor_buffer: Arc<Mutex<Vec<SensorData>>>,
}

impl RoombaService {
    pub fn init() -> RoombaService {
        RoombaService {
            sensor_buffer: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn push_sensor_data_to_buffer(&self, sensor_data: SensorData) {
        let buffer_clone = self.sensor_buffer.clone();
        buffer_clone.lock().unwrap().push(sensor_data);
    }

    pub fn pop_sensor_data_from_buffer(&self) -> Option<SensorData> {
        let mut sensor_buffer = self.sensor_buffer.lock().unwrap();
        if sensor_buffer.len() > 0 {
            Some(sensor_buffer.remove(0))
        } else {
            None
        }
    }
}

#[tonic::async_trait]
impl Roomba for RoombaService {
    async fn send_sensor_stream(
        &self,
        request: Request<tonic::Streaming<SensorData>>,
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
    type GetSensorDataStream = Pin<Box<dyn Stream<Item = Result<SensorData, Status>>
    + Send
    + Sync
    +'static >>;

    async fn get_sensor_data(
        &self,
        request: Request<SensorsRequest>,
    ) -> Result<Response<Self::GetSensorDataStream>, Status> {
        unimplemented!("todo")
    }
}
