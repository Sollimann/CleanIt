pub struct Model;
impl Model {
    pub const WHEEL_DIAMETER: f32 = 0.072; // m
    pub const AXLE_LENGTH: f32 = 0.235; // m
    pub const MAX_ENCODER_TICKS: i32 = 65535;
    pub const TICKS_PER_REV: f32 = 508.8;
}
