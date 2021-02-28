extern crate nalgebra as na;
use crate::slam::model::Roomba;
use crate::utils::constants::Constants;
use crate::utils::timer::Timer;
use crate::utils::wrapping::wrap_heading;
use na::{Matrix, Vector3};
use std::f64::consts::PI;

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

#[derive(Debug, Clone)]
pub struct Odometry {
    // timer
    timer: Timer,

    // odom
    pub pose: PoseWithCovariance,
    pub vel: TwistWithCovariance,

    // cumulative distance
    left_wheel_dist: f64,
    right_wheel_dist: f64,

    // velocity
    requested_left_vel: u8,
    requested_right_vel: u8,

    // ticks
    prev_ticks_left: u16,
    prev_ticks_right: u16,
}

impl Odometry {
    // @classmethod
    pub fn init(init_left_ticks: u16, init_right_ticks: u16) -> Odometry {
        Odometry {
            timer: Timer::init_time(),
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
            left_wheel_dist: 0.0,
            right_wheel_dist: 0.0,
            requested_left_vel: 0,
            requested_right_vel: 0,
            prev_ticks_left: init_left_ticks,
            prev_ticks_right: init_right_ticks,
        }
    }

    pub fn wrap_encoders(&mut self, curr_ticks_left: u16, curr_ticks_right: u16) -> (i32, i32) {
        const MAX: i32 = Roomba::MAX_ENCODER_TICKS as i32;
        const MAX_95: i32 = (0.95 * (MAX as f32)) as i32;
        const MIN_05: i32 = (0.05 * (MAX as f32)) as i32;

        let delta_ticks_left: i32 = match (curr_ticks_left as i32, self.prev_ticks_left as i32) {
            (curr, prev) if curr < MIN_05 && prev > MAX_95 => (MAX - prev) + curr,
            (curr, prev) if curr > MAX_95 && prev < MIN_05 => ((curr - MAX) - prev),
            (curr, prev) => curr - prev,
        };

        let delta_ticks_right: i32 = match (curr_ticks_right as i32, self.prev_ticks_right as i32) {
            (curr, prev) if curr < MIN_05 && prev > MAX_95 => (MAX - prev) + curr,
            (curr, prev) if curr > MAX_95 && prev < MIN_05 => ((curr - MAX) - prev),
            (curr, prev) => curr - prev,
        };

        self.prev_ticks_left = curr_ticks_left;
        self.prev_ticks_right = curr_ticks_right;

        (delta_ticks_left, delta_ticks_right)
    }

    pub fn compute_odom(&mut self, curr_ticks_left: u16, curr_ticks_right: u16) {
        // update timestamp
        let dt = self.timer.get_dt();

        // delta ticks on each wheel
        let (delta_ticks_left, delta_ticks_right) =
            self.wrap_encoders(curr_ticks_left, curr_ticks_right);

        // delta dist on each wheel
        let delta_left_wheel_dist: f64 =
            (delta_ticks_left as f64 / Roomba::TICKS_PER_REV) * Roomba::WHEEL_DIAMETER * PI;

        let delta_right_wheel_dist: f64 =
            (delta_ticks_right as f64 / Roomba::TICKS_PER_REV) * Roomba::WHEEL_DIAMETER * PI;

        // cumulative dist on each wheel
        self.left_wheel_dist += delta_left_wheel_dist;
        self.right_wheel_dist += delta_right_wheel_dist;

        // compute deltas
        let ds: f64 = (delta_right_wheel_dist + delta_left_wheel_dist) / 2.0;
        let dyaw: f64 = (delta_right_wheel_dist - delta_left_wheel_dist) / Roomba::AXLE_LENGTH;
        let dx = ds * (self.pose.yaw + dyaw / 2.0).cos();
        let dy = ds * (self.pose.yaw + dyaw / 2.0).sin();

        // compute new pose
        let p_old = Vector3::new(self.pose.x, self.pose.y, self.pose.yaw);
        let delta = Vector3::new(dx, dy, dyaw);
        let p_new = p_old + delta;

        //compute new velocity
        let v_new = if dt > Constants::EPSILON {
            Vector3::new(ds / dt, 0.0, dyaw / dt)
        } else {
            Vector3::new(0.0, 0.0, 0.0)
        };

        self.pose.x = p_new[0];
        self.pose.y = p_new[1];
        self.pose.yaw = wrap_heading(p_new[2]);
        self.vel.x = v_new[0];
        self.vel.y = v_new[1];
        self.vel.yaw = v_new[2];
    }
}
