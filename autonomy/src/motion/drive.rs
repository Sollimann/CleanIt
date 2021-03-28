use crate::motion::Roomba;
use crate::utils::saturate::saturate;
use drivers::roomba::drive::drive_direct;
use serialport::SerialPort;

/// https://snapcraft.io/blog/your-first-robot-the-driver-4-5
pub fn drive(x_vel: f64, ang_vel: f64, port: Box<dyn SerialPort>) -> Box<dyn SerialPort> {
    // Compute left and right wheel velocities
    let left_vel = x_vel - (Roomba::AXLE_LENGTH / 2.0) * ang_vel;
    let right_vel = x_vel + (Roomba::AXLE_LENGTH / 2.0) * ang_vel;

    let left_cmd: i16 = (left_vel * 1000.0).round() as i16;
    let right_cmd: i16 = (right_vel * 1000.0).round() as i16;

    let (left_cmd_sat, right_cmd_sat) = saturate(left_cmd, right_cmd, 500);

    drive_direct(left_cmd_sat, right_cmd_sat, port)
}
