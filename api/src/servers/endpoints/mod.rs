mod get_odometry_raw;
mod get_sensor_data;
mod send_sensor_stream;

// grpc theads
use crossbeam_channel::{bounded, Receiver, Sender};

// get custom protos
use proto::roomba_service_protos as protos;
use protos::SensorData;

#[derive(Debug)]
pub struct RoombaService {
    pub rx: Receiver<SensorData>,
    tx: Sender<SensorData>,
}

impl RoombaService {
    pub fn new() -> Self {
        let (tx, rx) = bounded(100);
        RoombaService { rx, tx }
    }
}
