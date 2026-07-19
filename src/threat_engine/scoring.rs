use crate::models::structs::{
    DroneType,
    TrackedDrone,
};

pub struct ThreatScorer;

impl ThreatScorer {

    /// Calculate total threat score
    pub fn calculate_score(drone: &TrackedDrone) -> f32 {

        Self::distance_score(drone.distance)
            + Self::speed_score(drone.speed)
            + Self::confidence_score(drone.confidence)
            + Self::drone_type_score(&drone.drone_type)
    }

    /// Distance Score (0-40)
    pub fn distance_score(distance: f32) -> f32 {

        if distance <= 50.0 {
            40.0
        } else if distance <= 100.0 {
            30.0
        } else if distance <= 200.0 {
            20.0
        } else if distance <= 300.0 {
            10.0
        } else {
            0.0
        }
    }

    /// Speed Score (0-20)
    pub fn speed_score(speed: f32) -> f32 {

        if speed >= 40.0 {
            20.0
        } else if speed >= 30.0 {
            15.0
        } else if speed >= 20.0 {
            10.0
        } else if speed >= 10.0 {
            5.0
        } else {
            0.0
        }
    }

    /// Detection Confidence Score (0-10)
    pub fn confidence_score(confidence: f32) -> f32 {

        if confidence >= 0.95 {
            10.0
        } else if confidence >= 0.90 {
            8.0
        } else if confidence >= 0.80 {
            5.0
        } else if confidence >= 0.70 {
            2.0
        } else {
            0.0
        }
    }

    /// Drone Type Score (0-30)
    pub fn drone_type_score(drone_type: &DroneType) -> f32 {

        match drone_type {

            DroneType::Attack => 30.0,

            DroneType::Swarm => 25.0,

            DroneType::Surveillance => 10.0,

            DroneType::Unknown => 15.0,
        }
    }

    /// Normalize score to 0–100
    pub fn normalize(score: f32) -> f32 {

        score.clamp(0.0, 100.0)
    }

    /// Return score as percentage
    pub fn score_percentage(score: f32) -> f32 {

        Self::normalize(score)
    }

    /// Check if score exceeds a threshold
    pub fn exceeds_threshold(
        score: f32,
        threshold: f32,
    ) -> bool {

        score >= threshold
    }
}