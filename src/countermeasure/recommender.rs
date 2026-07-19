use crate::threat_engine::analyzer::{
    DroneType,
    ThreatLevel,
    ThreatResult,
};

#[derive(Debug, Clone)]
pub enum CounterMeasure {
    Monitor,
    RFJammer,
    GPSJammer,
    Spoofing,
    LaserWeapon,
    MissileInterceptor,
    CIWS,
    SmokeScreen,
    Retreat,
}

#[derive(Debug)]
pub struct Recommendation {

    pub drone_type: DroneType,

    pub threat_level: ThreatLevel,

    pub threat_score: f32,

    pub counter_measure: CounterMeasure,

    pub priority: u8,

    pub confidence: f32,

    pub reason: String,
}

pub struct Recommender;

impl Recommender {

    pub fn recommend(threat: &ThreatResult) -> Recommendation {

        let (counter_measure, priority, reason) = match (
            &threat.drone_type,
            &threat.level,
        ) {

            (DroneType::Surveillance, ThreatLevel::Low) => (
                CounterMeasure::Monitor,
                1,
                "Drone is only performing surveillance."
            ),

            (DroneType::Surveillance, ThreatLevel::Medium) => (
                CounterMeasure::RFJammer,
                2,
                "Interrupt communication link."
            ),

            (DroneType::Attack, ThreatLevel::Medium) => (
                CounterMeasure::GPSJammer,
                3,
                "Disrupt navigation before engagement."
            ),

            (DroneType::Attack, ThreatLevel::High) => (
                CounterMeasure::LaserWeapon,
                4,
                "Neutralize approaching attack drone."
            ),

            (DroneType::Attack, ThreatLevel::Critical) => (
                CounterMeasure::MissileInterceptor,
                5,
                "Immediate interception required."
            ),

            (DroneType::Swarm, ThreatLevel::Medium) => (
                CounterMeasure::RFJammer,
                4,
                "Wide-area RF suppression."
            ),

            (DroneType::Swarm, ThreatLevel::High) => (
                CounterMeasure::CIWS,
                5,
                "Multiple drones require rapid engagement."
            ),

            (DroneType::Swarm, ThreatLevel::Critical) => (
                CounterMeasure::MissileInterceptor,
                5,
                "High-density swarm detected."
            ),

            _ => (
                CounterMeasure::Monitor,
                1,
                "Continue monitoring."
            ),
        };

        Recommendation {

            drone_type: threat.drone_type.clone(),

            threat_level: threat.level.clone(),

            threat_score: threat.score,

            counter_measure,

            priority,

            confidence: threat.confidence,

            reason: reason.to_string(),
        }
    }
}