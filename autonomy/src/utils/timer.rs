use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Timer {
    start: Instant,
    now: Instant,
}

impl Timer {
    pub fn init_time() -> Timer {
        let init = Instant::now();
        Timer {
            start: init,
            now: init,
        }
    }

    pub fn duration_since_start(&self) -> f64 {
        let new_now: Instant = Instant::now();
        let duration = new_now.duration_since(self.start);
        duration.as_secs_f64()
    }

    pub fn get_dt(&self) -> f64 {
        let new_now: Instant = Instant::now();
        let duration = new_now.duration_since(self.now);
        duration.as_secs_f64()
    }
}
