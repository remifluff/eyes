pub struct Timer {
    duration_sec: f32,
    last: f32,
}

impl Timer {
    pub fn start_new(time: f32, duration_sec: f32) -> Timer {
        Timer {
            duration_sec,
            last: time,
        }
    }

    pub fn check(&mut self, t: f32) -> bool {
        if t - self.last > self.duration_sec {
            self.last = t;
            true
        } else {
            false
        }
    }
}
