extern crate autonomy;

use autonomy::utils::timer::Timer;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_timer() {
    let mut timer = Timer::init_time();
    sleep(Duration::new(0, 0.1e+9 as u32));
    let duration = timer.duration_since_start();
    let dt = timer.get_dt();

    assert_eq!(true, duration < 1.0);
    assert_eq!(true, dt < 0.2);
    assert_eq!(true, dt >= 0.1);

    sleep(Duration::new(0, 0.3e+9 as u32));
    let duration = timer.duration_since_start();
    let dt = timer.get_dt();

    assert_eq!(true, duration < 1.0);
    assert_eq!(true, duration > 0.3);
    assert_eq!(true, dt > 0.2);
    assert_eq!(true, dt < 0.4);
}
