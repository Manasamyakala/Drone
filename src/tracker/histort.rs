use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TrackPoint {
    /// X coordinate (meters)
    pub x: f32,

    /// Y coordinate (meters)
    pub y: f32,

    /// Z coordinate (meters)
    pub z: f32,

    /// Timestamp (milliseconds)
    pub timestamp: u64,

    /// Estimated speed (m/s)
    pub speed: f32,
}

#[derive(Debug, Clone)]
pub struct TrackHistory {
    /// Drone ID
    pub drone_id: u32,

    /// Historical track points
    pub points: Vec<TrackPoint>,
}

pub struct HistoryTracker {
    /// History for all tracked drones
    histories: HashMap<u32, TrackHistory>,

    /// Maximum history length per drone
    max_history: usize,
}

impl HistoryTracker {

    /// Create a new history tracker
    pub fn new(max_history: usize) -> Self {
        Self {
            histories: HashMap::new(),
            max_history,
        }
    }

    /// Add a new tracking point
    pub fn add_point(
        &mut self,
        drone_id: u32,
        point: TrackPoint,
    ) {

        let history = self.histories.entry(drone_id)
            .or_insert(TrackHistory {
                drone_id,
                points: Vec::new(),
            });

        history.points.push(point);

        if history.points.len() > self.max_history {
            history.points.remove(0);
        }
    }

    /// Get history of one drone
    pub fn get_history(
        &self,
        drone_id: u32,
    ) -> Option<&TrackHistory> {

        self.histories.get(&drone_id)
    }

    /// Remove history when drone disappears
    pub fn remove(
        &mut self,
        drone_id: u32,
    ) {

        self.histories.remove(&drone_id);
    }

    /// Clear all histories
    pub fn clear(
        &mut self,
    ) {

        self.histories.clear();
    }

    /// Number of tracked drones
    pub fn tracked_count(
        &self,
    ) -> usize {

        self.histories.len()
    }

    /// Check if a drone exists
    pub fn contains(
        &self,
        drone_id: u32,
    ) -> bool {

        self.histories.contains_key(&drone_id)
    }

    /// Latest position
    pub fn latest_point(
        &self,
        drone_id: u32,
    ) -> Option<&TrackPoint> {

        self.histories
            .get(&drone_id)?
            .points
            .last()
    }

    /// Previous position
    pub fn previous_point(
        &self,
        drone_id: u32,
    ) -> Option<&TrackPoint> {

        let history = self.histories.get(&drone_id)?;

        if history.points.len() < 2 {
            return None;
        }

        history.points.get(history.points.len() - 2)
    }

    /// Calculate average speed
    pub fn average_speed(
        &self,
        drone_id: u32,
    ) -> Option<f32> {

        let history = self.histories.get(&drone_id)?;

        if history.points.is_empty() {
            return None;
        }

        let sum: f32 = history
            .points
            .iter()
            .map(|p| p.speed)
            .sum();

        Some(sum / history.points.len() as f32)
    }

    /// Get all histories
    pub fn all(
        &self,
    ) -> Vec<&TrackHistory> {

        self.histories.values().collect()
    }
}