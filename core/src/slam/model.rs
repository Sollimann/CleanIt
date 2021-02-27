pub struct Roomba;
impl Roomba {
    pub const WHEEL_DIAMETER: f32 = 0.072; // m
    pub const AXLE_LENGTH: f32 = 0.235; // m
    pub const MAX_ENCODER_TICKS: u16 = 65535;
    pub const TICKS_PER_REV: f32 = 508.8;
}
