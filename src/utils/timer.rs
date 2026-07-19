use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Timer {
    start: Instant,
}

impl Timer {
    /// Start a new timer
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    /// Restart the timer
    pub fn reset(&mut self) {
        self.start = Instant::now();
    }

    /// Elapsed time
    pub fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }

    /// Elapsed milliseconds
    pub fn elapsed_ms(&self) -> u128 {
        self.elapsed().as_millis()
    }

    /// Elapsed microseconds
    pub fn elapsed_us(&self) -> u128 {
        self.elapsed().as_micros()
    }

    /// Elapsed seconds
    pub fn elapsed_secs(&self) -> f32 {
        self.elapsed().as_secs_f32()
    }

    /// Frames per second
    pub fn fps(&self, frames: u32) -> f32 {
        let secs = self.elapsed_secs();

        if secs <= 0.0 {
            0.0
        } else {
            frames as f32 / secs
        }
    }
}

/// Simple stopwatch
pub struct Stopwatch {
    timer: Timer,
    running: bool,
}

impl Stopwatch {
    pub fn new() -> Self {
        Self {
            timer: Timer::new(),
            running: false,
        }
    }

    /// Start
    pub fn start(&mut self) {
        self.running = true;
        self.timer.reset();
    }

    /// Stop
    pub fn stop(&mut self) -> u128 {
        self.running = false;
        self.timer.elapsed_ms()
    }

    /// Check if running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Current elapsed time
    pub fn elapsed_ms(&self) -> u128 {
        self.timer.elapsed_ms()
    }
}