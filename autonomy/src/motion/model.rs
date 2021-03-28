pub struct Roomba;
impl Roomba {
    pub const WHEEL_DIAMETER: f64 = 0.072; // m
    pub const AXLE_LENGTH: f64 = 0.235; // m
    pub const MAX_ENCODER_TICKS: u16 = 65535;
    pub const TICKS_PER_REV: f64 = 508.8;
    pub const MAX_VELOCITY: f32 = 0.50; // m
}
