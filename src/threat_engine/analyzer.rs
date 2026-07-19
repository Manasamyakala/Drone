use crate::models::structs::{
    ThreatLevel,
    TrackedDrone,
};

#[derive(Debug, Clone)]
pub struct ThreatResult {
    pub drone_id: u32,
    pub threat_level: ThreatLevel,
    pub score: f32,
    pub reason: String,
}

pub struct ThreatAnalyzer;

impl ThreatAnalyzer {

    /// Analyze a single tracked drone
    pub fn analyze(
        drone: &TrackedDrone,
    ) -> ThreatResult {

        let mut score = 0.0;
        let mut reasons: Vec<String> = Vec::new();

        //-------------------------------------------------
        // Distance
        //-------------------------------------------------

        if drone.distance < 50.0 {
            score += 40.0;
            reasons.push("Very Close".to_string());
        }
        else if drone.distance < 150.0 {
            score += 25.0;
            reasons.push("Close".to_string());
        }
        else if drone.distance < 300.0 {
            score += 10.0;
        }

        //-------------------------------------------------
        // Speed
        //-------------------------------------------------

        if drone.speed > 35.0 {
            score += 20.0;
            reasons.push("High Speed".to_string());
        }
        else if drone.speed > 20.0 {
            score += 10.0;
        }

        //-------------------------------------------------
        // Detection Confidence
        //-------------------------------------------------

        if drone.confidence > 0.95 {
            score += 10.0;
        }
        else if drone.confidence > 0.80 {
            score += 5.0;
        }

        //-------------------------------------------------
        // Drone Type
        //-------------------------------------------------

        use crate::models::structs::DroneType;

        match drone.drone_type {

            DroneType::Attack => {
                score += 25.0;
                reasons.push("Attack Drone".to_string());
            }

            DroneType::Swarm => {
                score += 20.0;
                reasons.push("Swarm".to_string());
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

        let level =
            if score >= 80.0 {
                ThreatLevel::Critical
            }
            else if score >= 60.0 {
                ThreatLevel::High
            }
            else if score >= 35.0 {
                ThreatLevel::Medium
            }
            else {
                ThreatLevel::Low
            };

        ThreatResult {

            drone_id: drone.drone_id,

            threat_level: level,

            score,

            reason: reasons.join(", "),
        }
    }

    /// Analyze all tracked drones
    pub fn analyze_all(
        drones: &[TrackedDrone],
    ) -> Vec<ThreatResult> {

        drones
            .iter()
            .map(Self::analyze)
            .collect()
    }
}