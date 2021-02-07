// our messages and services
pub mod roombasensors {
    tonic::include_proto!("roombasensors");
}
use roombasensors::roomba_sensors_client::RoombaSensorsClient;
use roombasensors::{LightBumper, SensorRequest, Sensors, SensorsReceived, Stasis};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RoombaSensorsClient::connect("http://[::1]:10000").await?;

    Ok(())
}
