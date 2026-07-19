#[derive(Debug, Clone)]
pub struct KalmanFilter {
    /// State vector [x, y, vx, vy]
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,

    /// Time step
    dt: f32,

    /// Process noise
    process_noise: f32,

    /// Measurement noise
    measurement_noise: f32,

    /// Estimate uncertainty
    uncertainty: f32,
}

impl KalmanFilter {
    /// Create a new Kalman filter
    pub fn new(
        x: f32,
        y: f32,
        dt: f32,
    ) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            dt,
            process_noise: 1.0,
            measurement_noise: 2.0,
            uncertainty: 1.0,
        }
    }

    /// Predict the next state
    pub fn predict(&mut self) {
        self.x += self.vx * self.dt;
        self.y += self.vy * self.dt;

        self.uncertainty += self.process_noise;
    }

    /// Update using a measured position
    pub fn update(
        &mut self,
        measured_x: f32,
        measured_y: f32,
    ) {
        let gain = self.uncertainty
            / (self.uncertainty + self.measurement_noise);

        let old_x = self.x;
        let old_y = self.y;

        self.x += gain * (measured_x - self.x);
        self.y += gain * (measured_y - self.y);

        self.vx = (self.x - old_x) / self.dt;
        self.vy = (self.y - old_y) / self.dt;

        self.uncertainty *= 1.0 - gain;
    }

    /// Get current position
    pub fn position(&self) -> (f32, f32) {
        (self.x, self.y)
    }

    /// Get current velocity
    pub fn velocity(&self) -> (f32, f32) {
        (self.vx, self.vy)
    }

    /// Estimated speed
    pub fn speed(&self) -> f32 {
        (self.vx.powi(2) + self.vy.powi(2)).sqrt()
    }

    /// Reset the filter
    pub fn reset(
        &mut self,
        x: f32,
        y: f32,
    ) {
        self.x = x;
        self.y = y;
        self.vx = 0.0;
        self.vy = 0.0;
        self.uncertainty = 1.0;
    }

    /// Set process noise
    pub fn set_process_noise(
        &mut self,
        value: f32,
    ) {
        self.process_noise = value;
    }

    /// Set measurement noise
    pub fn set_measurement_noise(
        &mut self,
        value: f32,
    ) {
        self.measurement_noise = value;
    }
}