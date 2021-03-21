mod send_sensor_stream;

use std::sync::{Arc, Mutex};

// grpc
use tokio::sync::mpsc;
use tokio::sync::mpsc::{Receiver, Sender};

// get custom protos
use proto::roomba_service_protos as protos;
use protos::SensorData;

#[derive(Debug)]
pub struct RoombaService {
    pub sensor_buffer: Arc<Mutex<Vec<SensorData>>>,
    rx: Receiver<SensorData>,
    tx: Sender<SensorData>,
}

impl RoombaService {
    pub fn default() -> RoombaService {
        let (tx, rx) = mpsc::channel(100);
        RoombaService {
            sensor_buffer: Arc::new(Mutex::new(vec![])),
            rx,
            tx,
        }
    }
}
