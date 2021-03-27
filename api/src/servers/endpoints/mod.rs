mod get_odometry_raw;
mod get_sensor_data;
mod send_sensor_stream;

use std::sync::{Arc, Mutex};

// grpc theads
// use tokio::sync::mpsc;
// use tokio::sync::mpsc::{Receiver, Sender};
use crossbeam_channel::{bounded, Receiver, Sender};

// get custom protos
use proto::roomba_service_protos as protos;
use protos::SensorData;

#[derive(Debug)]
pub struct RoombaService {
    pub sensor_buffer: Arc<Mutex<Vec<SensorData>>>,
    pub rx: Receiver<SensorData>,
    tx: Sender<SensorData>,
}

impl RoombaService {
    pub fn new() -> Self {
        let (tx, rx) = bounded(100);
        RoombaService {
            sensor_buffer: Arc::new(Mutex::new(vec![])),
            rx,
            tx,
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
