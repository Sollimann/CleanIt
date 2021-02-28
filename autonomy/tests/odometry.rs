extern crate autonomy;

use autonomy::slam::odometry::Odometry;

#[test]
fn test_odometry_init() {
    let mut odom = Odometry::init(0, 0);
    let (left, right) = odom.wrap_encoders(100, 100);

    assert_eq!(left, 100);
    assert_eq!(right, 100);
}

#[test]
fn test_encoder_wrapping_forward() {
    let mut odom = Odometry::init(65500, 65500);
    let (left, right) = odom.wrap_encoders(200, 200);

    assert_eq!(left, 235);
    assert_eq!(right, 235);
}

#[test]
fn test_encoder_wrapping_backward() {
    let mut odom = Odometry::init(200, 200);
    let (left, right) = odom.wrap_encoders(65500, 65500);

    assert_eq!(left, -235);
    assert_eq!(right, -235);
}

#[test]
fn test_encoder_wrapping_backward_then_forward() {
    let mut odom = Odometry::init(200, 200);
    let (left_0, right_0) = odom.wrap_encoders(65500, 65500);

    assert_eq!(left_0, -235);
    assert_eq!(right_0, -235);

    let (left_1, right_1) = odom.wrap_encoders(200, 200);
    assert_eq!(left_1, 235);
    assert_eq!(right_1, 235);
}

#[test]
fn test_compute_odom() {
    let mut odom = Odometry::init(200, 200);
    odom.compute_odom(5000, 6000);

    println!("pose: {:?}", odom.pose);
    println!("vel: {:?}", odom.vel);
}
