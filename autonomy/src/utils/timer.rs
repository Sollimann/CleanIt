#![allow(dead_code)]
use std::time::Instant;

#[derive(Debug, Clone)]
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

    pub fn get_dt(&mut self) -> f64 {
        let new_now: Instant = Instant::now();
        let duration = new_now.duration_since(self.now);
        self.now = new_now;
        duration.as_secs_f64()
    }
}
