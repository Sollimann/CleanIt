use crate::utils::timer::Timer;

pub struct Roomba;
impl Roomba {
    pub const WHEEL_DIAMETER: f64 = 0.072; // m
    pub const AXLE_LENGTH: f64 = 0.235; // m
    pub const MAX_ENCODER_TICKS: u16 = 65535;
    pub const TICKS_PER_REV: f64 = 508.8;
}

#[derive(Debug, Clone)] // needed for printing the object
pub struct PoseWithCovariance {
    pub x: f64,
    pub y: f64,
    pub yaw: f64,
    pub covariance: [f64; 9], // 3x3 matrix represented as an array
}

#[derive(Debug, Clone)]
pub struct TwistWithCovariance {
    pub x: f64,
    pub y: f64,
    pub yaw: f64,
    pub covariance: [f64; 9], // 3x3 matrix represented as an array
}
