use crate::models::structs::{
    CounterMeasure,
    DroneType,
    Recommendation,
    ThreatLevel,
    ThreatResult,
};

pub struct Recommender;

impl Recommender {
    pub fn recommend(threat: &ThreatResult) -> Recommendation {
        let (counter_measure, priority, reason) = match (
            &threat.drone_type,
            &threat.threat_level,
        ) {
            (DroneType::Surveillance, ThreatLevel::Low) => (
                CounterMeasure::Monitor,
                1,
                "Drone is only performing surveillance.",
            ),

            (DroneType::Surveillance, ThreatLevel::Medium) => (
                CounterMeasure::RFJammer,
                2,
                "Interrupt communication link.",
            ),

            (DroneType::Attack, ThreatLevel::Low) => (
                CounterMeasure::RFJammer,
                2,
                "Attack drone detected. Begin electronic countermeasures.",
            ),

            (DroneType::Attack, ThreatLevel::Medium) => (
                CounterMeasure::GPSJammer,
                3,
                "Disrupt navigation before engagement.",
            ),

            (DroneType::Attack, ThreatLevel::High) => (
                CounterMeasure::LaserWeapon,
                4,
                "Neutralize approaching attack drone.",
            ),

            (DroneType::Attack, ThreatLevel::Critical) => (
                CounterMeasure::MissileInterceptor,
                5,
                "Immediate interception required.",
            ),

            (DroneType::Swarm, ThreatLevel::Low) => (
                CounterMeasure::RFJammer,
                3,
                "Possible swarm activity detected.",
            ),

            (DroneType::Swarm, ThreatLevel::Medium) => (
                CounterMeasure::RFJammer,
                4,
                "Wide-area RF suppression.",
            ),

            (DroneType::Swarm, ThreatLevel::High) => (
                CounterMeasure::CIWS,
                5,
                "Multiple drones require rapid engagement.",
            ),

            (DroneType::Swarm, ThreatLevel::Critical) => (
                CounterMeasure::MissileInterceptor,
                5,
                "High-density swarm detected.",
            ),

            (DroneType::Unknown, ThreatLevel::Critical) => (
                CounterMeasure::MissileInterceptor,
                5,
                "Unknown drone posing critical threat.",
            ),

            (DroneType::Unknown, ThreatLevel::High) => (
                CounterMeasure::LaserWeapon,
                4,
                "Unknown hostile drone.",
            ),

            _ => (
                CounterMeasure::Monitor,
                1,
                "Continue monitoring.",
            ),
        };

        Recommendation {
            drone_id: threat.drone_id,
            counter_measure,
            priority,
            confidence: threat.confidence,
            reason: format!(
                "{} (Threat Score: {:.1})",
                reason,
                threat.threat_score
            ),
        }
    }
}