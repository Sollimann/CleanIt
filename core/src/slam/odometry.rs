use crate::slam::model::Model;
use nalgebra::{Rotation3, Vector2};

struct Pose {
    x: f32,
    y: f32,
    yaw: f32,
}

pub struct Odometry {
    // timer
    current_time: i64,
    prev_time: i64,
    dt: i64,

    // distance
    dx: f32,
    dy: f32,
    dyaw: f32,
    delta_dist: f32,
    left_wheel_dist: f32,
    right_wheel_dist: f32,
    wheel_dist_diff: f32,

    // velocity
    requested_left_vel: u8,
    requested_right_vel: u8,

    // ticks
    prev_ticks_left: u16,
    prev_ticks_right: u16,
    total_left_ticks: u32,
    total_right_ticks: u32,
}

impl Odometry {
    fn dummy() {
        let a = Model::WHEEL_DIAMETER;
    }
}
