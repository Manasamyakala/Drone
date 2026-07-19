use crate::tracker::history::{TrackHistory, TrackPoint};

#[derive(Debug, Clone)]
pub struct PredictedPosition {
    /// Predicted X coordinate
    pub x: f32,

    /// Predicted Y coordinate
    pub y: f32,

    /// Predicted Z coordinate
    pub z: f32,

    /// Prediction time (seconds ahead)
    pub prediction_time: f32,
}

pub struct TrajectoryPredictor;

impl TrajectoryPredictor {

    /// Predict future position using the last two tracking points
    pub fn predict(
        history: &TrackHistory,
        seconds_ahead: f32,
    ) -> Option<PredictedPosition> {

        if history.points.len() < 2 {
            return None;
        }

        let last = history.points.last().unwrap();
        let previous = &history.points[history.points.len() - 2];

        let dt =
            (last.timestamp - previous.timestamp) as f32 / 1000.0;

        if dt <= 0.0 {
            return None;
        }

        let vx = (last.x - previous.x) / dt;
        let vy = (last.y - previous.y) / dt;
        let vz = (last.z - previous.z) / dt;

        Some(PredictedPosition {

            x: last.x + vx * seconds_ahead,

            y: last.y + vy * seconds_ahead,

            z: last.z + vz * seconds_ahead,

            prediction_time: seconds_ahead,
        })
    }

    /// Calculate current velocity
    pub fn velocity(
        history: &TrackHistory,
    ) -> Option<(f32, f32, f32)> {

        if history.points.len() < 2 {
            return None;
        }

        let last = history.points.last().unwrap();
        let previous = &history.points[history.points.len() - 2];

        let dt =
            (last.timestamp - previous.timestamp) as f32 / 1000.0;

        if dt <= 0.0 {
            return None;
        }

        Some((
            (last.x - previous.x) / dt,
            (last.y - previous.y) / dt,
            (last.z - previous.z) / dt,
        ))
    }

    /// Calculate current speed
    pub fn speed(
        history: &TrackHistory,
    ) -> Option<f32> {

        let (vx, vy, vz) = Self::velocity(history)?;

        Some((vx * vx + vy * vy + vz * vz).sqrt())
    }

    /// Calculate heading angle (degrees)
    pub fn heading(
        history: &TrackHistory,
    ) -> Option<f32> {

        let (vx, vy, _) = Self::velocity(history)?;

        Some(vy.atan2(vx).to_degrees())
    }

    /// Predict multiple future positions
    pub fn predict_path(
        history: &TrackHistory,
        interval: f32,
        steps: usize,
    ) -> Vec<PredictedPosition> {

        let mut predictions = Vec::new();

        for i in 1..=steps {

            let time = interval * i as f32;

            if let Some(point) =
                Self::predict(history, time)
            {
                predictions.push(point);
            }
        }

        predictions
    }
}