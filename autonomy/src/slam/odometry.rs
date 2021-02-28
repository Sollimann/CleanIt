#![allow(dead_code)]
extern crate nalgebra as na;
use crate::slam::model::Roomba;
use crate::utils::constants::Constants;
use crate::utils::matrix::{
    covar_to_matrix3, mat_multiply_3x2_2x2_2x3, mat_multiply_3x3_3x3_3x3, matrix3_to_covar,
};
use crate::utils::timer::Timer;
use crate::utils::wrapping::wrap_heading;
use na::{Matrix2, Matrix3, Matrix3x2, RowVector2, RowVector3, Vector3};
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
                covariance: [1e-9, 1e-9, 1e-9, 1e-9, 1e-9, 1e-9, 1e-9, 1e-9, 1e-9],
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

    pub fn compute_odom(&mut self, curr_ticks_left: u16, curr_ticks_right: u16) -> &mut Odometry {
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
        let ds_2b = ds / (2.0 * Roomba::AXLE_LENGTH);
        let dyaw: f64 = (delta_right_wheel_dist - delta_left_wheel_dist) / Roomba::AXLE_LENGTH;
        let c_yaw = (self.pose.yaw + (dyaw / 2.0)).cos();
        let s_yaw = (self.pose.yaw + (dyaw / 2.0)).sin();
        let dx = ds * c_yaw;
        let dy = ds * s_yaw;

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

        // compute velocity/motion covariance
        let kr = 1.0;
        let kl = 1.0;
        let i_00 = kr * delta_right_wheel_dist.abs();
        let i_11 = kl * delta_left_wheel_dist.abs();

        #[rustfmt::skip]
        let vel_covar = Matrix2::from_rows(&[
                RowVector2::new(i_00, 0.0),
                RowVector2::new(0.0, i_11)]
        );

        // compute velocity Jacobian
        let i_00 = (c_yaw / 2.0) - (ds_2b * s_yaw);
        let i_01 = (c_yaw / 2.0) + (ds_2b * s_yaw);
        let i_10 = (s_yaw / 2.0) + (ds_2b * c_yaw);
        let i_11 = (s_yaw / 2.0) - (ds_2b * c_yaw);
        let i_20 = 1.0 / Roomba::AXLE_LENGTH;
        let i_21 = -i_20;

        let vel_jacob = Matrix3x2::from_rows(&[
            RowVector2::new(i_00, i_01),
            RowVector2::new(i_10, i_11),
            RowVector2::new(i_20, i_21),
        ]);

        // 3x3 = 3x2 * (2x2 * 2*3)
        let vel_covar_matrix_est: Matrix3<f64> = mat_multiply_3x2_2x2_2x3(vel_jacob, vel_covar);

        // compute pose jacobian
        let i_00 = 1.0;
        let i_01 = 0.0;
        let i_02 = -dy;
        let i_10 = 0.0;
        let i_11 = 1.0;
        let i_12 = dx;
        let i_20 = 0.0;
        let i_21 = 0.0;
        let i_22 = 1.0;

        let pose_jacob = Matrix3::from_rows(&[
            RowVector3::new(i_00, i_01, i_02),
            RowVector3::new(i_10, i_11, i_12),
            RowVector3::new(i_20, i_21, i_22),
        ]);

        // return current covariance
        let pose_covar: Matrix3<f64> = covar_to_matrix3(self.pose.covariance);

        // 3x3 = 3x3 * (3x3 * 3*3)
        let pose_covar_matrix_est: Matrix3<f64> = mat_multiply_3x3_3x3_3x3(pose_jacob, pose_covar);

        // update odom
        self.pose.x = p_new[0];
        self.pose.y = p_new[1];
        self.pose.yaw = wrap_heading(p_new[2]);
        self.pose.covariance = matrix3_to_covar(pose_covar_matrix_est);
        self.vel.x = v_new[0];
        self.vel.y = v_new[1];
        self.vel.yaw = v_new[2];
        self.vel.covariance = matrix3_to_covar(vel_covar_matrix_est);

        self
    }
}
