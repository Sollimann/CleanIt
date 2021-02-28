extern crate autonomy;

use autonomy::utils::timer::Timer;
use std::thread::sleep;
use std::time::Duration;

#[test]
fn test_timer() {
    let timer = Timer::init_time();
    sleep(Duration::new(0, 1e-3 as u32));
    let mut duration = timer.duration_since_start();
    let mut dt = timer.get_dt();

    assert_eq!(true, duration < 1.0);
    assert_eq!(true, dt < 1.0);

    sleep(Duration::new(1, 1e-3 as u32));
    let mut duration = timer.duration_since_start();
    let mut dt = timer.get_dt();

    assert_eq!(true, duration > 1.0);
    assert_eq!(true, dt > 1.0);
}
