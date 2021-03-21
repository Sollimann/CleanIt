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
    pub(crate) sensor_buffer: Arc<Mutex<Vec<SensorData>>>,
    rx: Receiver<SensorData>,
    tx: Sender<SensorData>,
}

impl RoombaService {
    pub fn default() -> RoombaService {
        let (tx, rx) = bounded(100);
        RoombaService {
            sensor_buffer: Arc::new(Mutex::new(vec![])),
            rx,
            tx,
        }
    }
}
