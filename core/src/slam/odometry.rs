use crate::slam::model::Roomba;
use nalgebra::{Rotation3, Vector2};

#[derive(Debug)] // needed for printing the object
struct PoseWithCovariance {
    x: f64,
    y: f64,
    yaw: f64,
    covariance: [f64; 9], // 3x3 matrix represented as an array
}

#[derive(Debug)]
struct TwistWithCovariance {
    x: f64,
    y: f64,
    yaw: f64,
    covariance: [f64; 9], // 3x3 matrix represented as an array
}

#[derive(Debug)]
pub struct Odometry {
    // timer
    current_time: i64,
    prev_time: i64,
    dt: i64,

    // odom
    pose: PoseWithCovariance,
    vel: TwistWithCovariance,

    // distance
    dx: f32,
    dy: f32,
    d_theta: f32,
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
        let a = Roomba::WHEEL_DIAMETER;
    }

    // @classmethod
    pub fn init(init_left_ticks: u16, init_right_ticks: u16, init_time: i64) -> Odometry {
        Odometry {
            current_time: init_time,
            prev_time: init_time,
            dt: 0,
            pose: PoseWithCovariance {
                x: 0.0,
                y: 0.0,
                yaw: 0.0,
                covariance: [0f64; 9],
            },
            vel: TwistWithCovariance {
                x: 0.0,
                y: 0.0,
                yaw: 0.0,
                covariance: [0f64; 9],
            },
            dx: 0.0,
            dy: 0.0,
            d_theta: 0.0,
            delta_dist: 0.0,
            left_wheel_dist: 0.0,
            right_wheel_dist: 0.0,
            wheel_dist_diff: 0.0,
            requested_left_vel: 0,
            requested_right_vel: 0,
            prev_ticks_left: init_left_ticks,
            prev_ticks_right: init_right_ticks,
            total_left_ticks: 0,
            total_right_ticks: 0,
        }
    }

    pub fn wrap_encoders(&self, curr_ticks_left: u16, curr_ticks_right: u16) -> (i16, i16) {
        const MAX: u16 = Roomba::MAX_ENCODER_TICKS;
        const MAX_95: u16 = (0.95 * (MAX as f32)) as u16;
        const MIN_05: u16 = (0.05 * (MAX as f32)) as u16;

        let delta_ticks_left: i16 = match (curr_ticks_left as u16, self.prev_ticks_left as u16) {
            (curr, prev) if curr < MIN_05 && prev > MAX_95 => (MAX - prev) + curr,
            (curr, prev) if curr > MAX_95 && prev < MIN_05 => ((curr - MAX) - prev),
            (curr, prev) => curr - prev,
        } as i16;

        let delta_ticks_right: i16 = match (curr_ticks_right as u16, self.prev_ticks_right as u16) {
            (curr, prev) if curr < MIN_05 && prev > MAX_95 => (MAX - prev) + curr,
            (curr, prev) if curr > MAX_95 && prev < MIN_05 => ((curr - MAX) - prev),
            (curr, prev) => curr - prev,
        } as i16;

        (delta_ticks_left, delta_ticks_right)
    }

    pub fn compute_wheel_distance(&self, curr_ticks_left: u16, curr_ticks_right: u16) {
        // let delta_ticks_left: u16 = curr_ticks_left - self.prev_ticks_left;
        // let delta_ticks_right: u16 = curr_ticks_right - self.prev_ticks_right;

        let (delta_ticks_left, delta_ticks_right) =
            self.wrap_encoders(curr_ticks_left, curr_ticks_right);
    }
}
