use std::collections::HashMap;

use crate::models::structs::{
    BoundingBox,
    DroneType,
    TrackedDrone,
};

use crate::tracker::history::{HistoryTracker, TrackPoint};
use crate::tracker::kalman::KalmanFilter;

pub struct Tracker {
    /// Active tracked drones
    tracks: HashMap<u32, TrackedDrone>,

    /// Kalman filter for each drone
    filters: HashMap<u32, KalmanFilter>,

    /// Track history
    history: HistoryTracker,

    /// Next available ID
    next_id: u32,
}

impl Tracker {
    /// Create tracker
    pub fn new() -> Self {
        Self {
            tracks: HashMap::new(),
            filters: HashMap::new(),
            history: HistoryTracker::new(30),
            next_id: 1,
        }
    }

    /// Register a newly detected drone
    pub fn register(
        &mut self,
        bbox: BoundingBox,
        confidence: f32,
        drone_type: DroneType,
    ) -> u32 {

        let id = self.next_id;
        self.next_id += 1;

        let center_x = bbox.x + bbox.width / 2.0;
        let center_y = bbox.y + bbox.height / 2.0;

        let drone = TrackedDrone {
            drone_id: id,
            drone_type,
            confidence,
            bbox: bbox.clone(),
            speed: 0.0,
            distance: 0.0,
            direction: "Unknown".to_string(),
        };

        self.tracks.insert(id, drone);

        self.filters
            .insert(id, KalmanFilter::new(center_x, center_y, 0.1));

        id
    }

    /// Update an existing track
    pub fn update(
        &mut self,
        drone_id: u32,
        bbox: BoundingBox,
    ) {

        if let Some(drone) = self.tracks.get_mut(&drone_id) {

            drone.bbox = bbox.clone();

            let center_x = bbox.x + bbox.width / 2.0;
            let center_y = bbox.y + bbox.height / 2.0;

            if let Some(filter) = self.filters.get_mut(&drone_id) {

                filter.predict();
                filter.update(center_x, center_y);

                let (_, _) = filter.position();
                let (vx, vy) = filter.velocity();

                drone.speed = filter.speed();

                drone.direction = direction(vx, vy);

                self.history.add_point(
                    drone_id,
                    TrackPoint {
                        x: center_x,
                        y: center_y,
                        z: 0.0,
                        timestamp: current_timestamp(),
                        speed: drone.speed,
                    },
                );
            }
        }
    }

    /// Remove a lost track
    pub fn remove(
        &mut self,
        drone_id: u32,
    ) {

        self.tracks.remove(&drone_id);

        self.filters.remove(&drone_id);

        self.history.remove(drone_id);
    }

    /// Get one tracked drone
    pub fn get(
        &self,
        drone_id: u32,
    ) -> Option<&TrackedDrone> {

        self.tracks.get(&drone_id)
    }

    /// Get all tracked drones
    pub fn all(
        &self,
    ) -> Vec<TrackedDrone> {

        self.tracks
            .values()
            .cloned()
            .collect()
    }

    /// Number of active tracks
    pub fn count(
        &self,
    ) -> usize {

        self.tracks.len()
    }

    /// Remove every track
    pub fn clear(
        &mut self,
    ) {

        self.tracks.clear();

        self.filters.clear();

        self.history.clear();
    }
}

/// Estimate movement direction
fn direction(
    vx: f32,
    vy: f32,
) -> String {

    if vx.abs() < 0.01 && vy.abs() < 0.01 {

        "Stationary".to_string()

    } else if vx.abs() > vy.abs() {

        if vx > 0.0 {
            "East".to_string()
        } else {
            "West".to_string()
        }

    } else {

        if vy > 0.0 {
            "North".to_string()
        } else {
            "South".to_string()
        }
    }
}

/// Current timestamp (milliseconds)
fn current_timestamp() -> u64 {

    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64
}