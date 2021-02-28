use autonomy::utils::constants::Constants;
use autonomy::utils::wrapping::wrap_heading;

#[test]
fn test_wrap_heading() {
    let unwrapped_heading = 4.1 * Constants::TWO_PI;
    let wrapped_heading = wrap_heading(unwrapped_heading);

    assert_eq!(true, wrapped_heading > -Constants::PI);
    assert_eq!(true, wrapped_heading < Constants::PI);
}
