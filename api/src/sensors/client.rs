use std::error::Error;

//
use futures::stream;
use tonic::transport::Channel;

// our messages and services
pub mod roombasensors {
    tonic::include_proto!("roombasensors");
}
use roombasensors::roomba_sensors_client::RoombaSensorsClient;
use roombasensors::{LightBumper, SensorRequest, Sensors, SensorsReceived, Stasis};
use tonic::Request;

async fn run_sensor_stream(
    client: &mut RoombaSensorsClient<Channel>,
) -> Result<(), Box<dyn Error>> {
    let mut sensor_readings = vec![];
    for _ in 0..100 {
        sensor_readings.push(random_sensors_values())
    }

    // create the request
    let request = Request::new(stream::iter(sensor_readings));

    match client.send_sensor_stream(request).await {
        Ok(response) => println!("RESPONSE: {:?}", response.into_inner()),
        Err(e) => println!("Something went wrong: {:?}", e),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = RoombaSensorsClient::connect("http://[::1]:10000").await?;

    println!("\n*** CLIENT STREAMING ***");
    run_sensor_stream(&mut client).await?;

    Ok(())
}

fn random_sensors_values() -> Sensors {
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

    Sensors {
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
