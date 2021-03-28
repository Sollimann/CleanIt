pub fn saturate(mut left_cmd: i16, mut right_cmd: i16, limit: i16) -> (i16, i16) {
    if left_cmd.abs() > limit {
        left_cmd = limit * (left_cmd.signum() as i16)
    }

    if right_cmd.abs() > limit {
        right_cmd = limit * (left_cmd.signum() as i16)
    }

    (left_cmd, right_cmd)
}
