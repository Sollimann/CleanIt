use futures::stream;
use std::error::Error;
use tonic::transport::Channel;
use tonic::Request;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let mut client = RoombaSensorsClient::connect("http://[::1]:10000").await?;

    println!("\n*** CLIENT STREAMING ***");
    // run_sensor_stream(&mut client).await?;
    //
    Ok(())
}

// fn random_sensors_values() -> Sensors {
//     let light_bumper_ex = LightBumper {
//         bumper_left: false,
//         bumper_front_left: true,
//         bumper_center_left: true,
//         bumper_center_right: false,
//         bumper_front_right: false,
//         bumper_right: false,
//     };
//
//     let stasis_ex = Stasis {
//         toggling: 0,
//         disabled: 1,
//     };
//
//     Sensors {
//         virtual_wall: false,
//         charging_state: 1,
//         voltage: 12345,
//         temperature: 18,
//         battery_charge: 1000,
//         battery_capacity: 2000,
//         oi_mode: 3,
//         requested_velocity: 50,
//         requested_radius: 200,
//         requested_right_velocity: 100,
//         requested_left_velocity: 100,
//         left_encoder_counts: 1111,
//         right_encoder_counts: 1245,
//         light_bumper: Some(light_bumper_ex),
//         stasis: Some(stasis_ex),
//     }
// }
