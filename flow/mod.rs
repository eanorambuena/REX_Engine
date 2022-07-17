use std::time::{Duration, Instant};

pub struct Frame {
    pub start_time: Instant,
    pub delta_time: Duration,
    pub fps: u32,
}

impl Frame {
    pub fn new(fps: u32) -> Frame {
        let start = Instant::now();
        Frame {
            start_time: start.clone(),
            delta_time: start.elapsed(),
            fps: fps,
        }
    }
    pub fn start(&mut self) {
        self.start_time = Instant::now();
    }
    pub fn end(&mut self) {
        self.delta_time = self.start_time.elapsed();
    }
    pub fn left_delta_time(&self) -> Duration {
        let frame_duration = Duration::from_millis((1.0 / (self.fps as f64) * 1000.0) as u64);
        frame_duration.saturating_sub(self.delta_time)
    }
}
