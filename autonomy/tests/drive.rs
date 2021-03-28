use autonomy::utils::saturate::saturate;

#[test]
fn test_saturate() {
    let left_cmd = 600;
    let right_cmd = 302;
    let limit = 500;

    let (left_sat, right_sat) = saturate(left_cmd, right_cmd, limit);

    assert_eq!(left_sat, limit);
    assert_eq!(right_sat, right_cmd);
}

#[test]
fn test_saturate_negative() {
    let left_cmd = -600;
    let right_cmd = -302;
    let limit = 500;

    let (left_sat, right_sat) = saturate(left_cmd, right_cmd, limit);

    assert_eq!(left_sat, -limit);
    assert_eq!(right_sat, right_cmd);
}
