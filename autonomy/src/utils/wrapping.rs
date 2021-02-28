use crate::utils::constants::Constants;
use std::f64::consts::PI;

pub fn wrap_heading(yaw: f64) -> f64 {
    let mut angle = yaw;

    while angle < -PI {
        angle += Constants::TWO_PI
    }
    while angle > PI {
        angle -= Constants::TWO_PI
    }

    angle
}
