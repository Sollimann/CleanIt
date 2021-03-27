extern crate autonomy;

use autonomy::slam::odometry::odometry::OdometryStamped;
use std::intrinsics::copy;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_odometry_init() {
    let mut odom = OdometryStamped::init(0, 0);
    let (left, right) = odom.wrap_encoders(100, 100);

    assert_eq!(left, 100);
    assert_eq!(right, 100);
}

#[test]
fn test_encoder_wrapping_forward() {
    let mut odom = OdometryStamped::init(65500, 65500);
    let (left, right) = odom.wrap_encoders(200, 200);

    assert_eq!(left, 235);
    assert_eq!(right, 235);
}

#[test]
fn test_encoder_wrapping_backward() {
    let mut odom = OdometryStamped::init(200, 200);
    let (left, right) = odom.wrap_encoders(65500, 65500);

    assert_eq!(left, -235);
    assert_eq!(right, -235);
}

#[test]
fn test_encoder_wrapping_backward_then_forward() {
    let mut odom = OdometryStamped::init(200, 200);
    let (left_0, right_0) = odom.wrap_encoders(65500, 65500);

    assert_eq!(left_0, -235);
    assert_eq!(right_0, -235);

    let (left_1, right_1) = odom.wrap_encoders(200, 200);
    assert_eq!(left_1, 235);
    assert_eq!(right_1, 235);
}

#[test]
fn test_compute_odom_driving_straight() {
    let mut odom = OdometryStamped::init(200, 200);
    sleep(Duration::new(0, 0.5e+9 as u32));
    odom.compute_odom(1200, 1200);

    let prev_odom = odom.clone();

    assert_eq!(true, prev_odom.pose.y - f64::EPSILON < 0.00000001);
    assert_eq!(true, prev_odom.pose.yaw - f64::EPSILON < 0.00000001);
    assert_eq!(true, prev_odom.vel.y - f64::EPSILON < 0.00000001);
    assert_eq!(true, prev_odom.vel.yaw - f64::EPSILON < 0.00000001);
    assert_eq!(true, prev_odom.vel.x > prev_odom.pose.x);

    sleep(Duration::new(0, 0.5e+9 as u32));
    odom.compute_odom(1400, 1400);

    assert_eq!(true, odom.pose.x > prev_odom.pose.x);
    assert_eq!(true, odom.vel.x < prev_odom.vel.x);
}

#[test]
fn test_compute_odom_driving_counter_clockwise() {
    let mut odom = OdometryStamped::init(200, 200);
    sleep(Duration::new(0, 0.5e+9 as u32));
    odom.compute_odom(1200, 2000);

    assert_eq!(true, odom.pose.y > 0.4);
    assert_eq!(true, odom.pose.yaw > 1.5);
    assert_eq!(true, odom.vel.y - f64::EPSILON < 0.00000001);
    assert_eq!(true, odom.vel.yaw > 3.0);
}

#[test]
fn test_compute_odom_driving_180_clockwise() {
    let mut odom = OdometryStamped::init(200, 200);
    sleep(Duration::new(0, 1e+9 as u32));
    odom.compute_odom(3800, 2200);

    assert_eq!(true, odom.pose.y < -1.0);
    assert_eq!(true, odom.pose.yaw < -3.0);
    assert_eq!(true, odom.vel.y - f64::EPSILON < 0.00000001);
    assert_eq!(true, odom.vel.yaw < -3.0);

    sleep(Duration::new(0, 0.3e+9 as u32));
    odom.compute_odom(3900, 2200);
    assert_eq!(true, odom.pose.yaw > 0.0);

    sleep(Duration::new(0, 1e+9 as u32));
    odom.compute_odom(6900, 5200);

    assert_eq!(true, odom.pose.x < -1.0);
    assert_eq!(true, odom.pose.y < -1.0);
    assert_eq!(true, odom.pose.yaw > 0.0);
}

#[test]
fn test_compute_odom_driving_forward_then_backward() {
    let mut odom = OdometryStamped::init(200, 200);
    sleep(Duration::new(0, 1e+9 as u32));
    odom.compute_odom(1000, 1000);

    assert_eq!(true, odom.pose.x > 0.3);

    sleep(Duration::new(0, 3e+9 as u32));
    odom.compute_odom(64000, 64000);

    assert_eq!(true, odom.pose.x < -0.7);
}
