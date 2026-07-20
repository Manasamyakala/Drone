use crate::models::structs::{
    DroneType,
    ThreatLevel,
    TrackedDrone,
    ThreatResult,
};

pub struct ThreatAnalyzer;

impl ThreatAnalyzer {
    /// Analyze a single tracked drone
    pub fn analyze(drone: &TrackedDrone) -> ThreatResult {
        let mut score = 0.0;

        //-------------------------------------------------
        // Distance
        //-------------------------------------------------

        if drone.distance < 50.0 {
            score += 40.0;
        } else if drone.distance < 150.0 {
            score += 25.0;
        } else if drone.distance < 300.0 {
            score += 10.0;
        }

        //-------------------------------------------------
        // Speed
        //-------------------------------------------------

        if drone.speed > 35.0 {
            score += 20.0;
        } else if drone.speed > 20.0 {
            score += 10.0;
        }

        //-------------------------------------------------
        // Detection Confidence
        //-------------------------------------------------

        if drone.confidence > 0.95 {
            score += 10.0;
        } else if drone.confidence > 0.80 {
            score += 5.0;
        }

        //-------------------------------------------------
        // Drone Type
        //-------------------------------------------------

        match drone.drone_type {
            DroneType::Attack => {
                score += 25.0;
            }

            DroneType::Swarm => {
                score += 20.0;
            }

            DroneType::Surveillance => {
                score += 5.0;
            }

            DroneType::Unknown => {
                score += 10.0;
            }
        }

        //-------------------------------------------------
        // Final Threat Level
        //-------------------------------------------------

        let threat_level = if score >= 80.0 {
            ThreatLevel::Critical
        } else if score >= 60.0 {
            ThreatLevel::High
        } else if score >= 35.0 {
            ThreatLevel::Medium
        } else {
            ThreatLevel::Low
        };

        ThreatResult {
            drone_id: drone.drone_id,
            drone_type: drone.drone_type.clone(),
            threat_level,
            threat_score: score,
            confidence: drone.confidence,
            distance: drone.distance,
            speed: drone.speed,
        }
    }

    /// Analyze all tracked drones
    pub fn analyze_all(drones: &[TrackedDrone]) -> Vec<ThreatResult> {
        drones.iter().map(Self::analyze).collect()
    }
}