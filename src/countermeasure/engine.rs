use crate::models::structs::{
    CounterMeasure,
    ThreatLevel,
    ThreatResult,
};

#[derive(Debug)]
pub struct CounterResponse {
    pub counter_measure: CounterMeasure,
    pub priority: u8,
    pub reason: String,
}

pub struct CounterMeasureEngine;

impl CounterMeasureEngine {
    pub fn recommend(threat: &ThreatResult) -> CounterResponse {
        match threat.threat_level {
            ThreatLevel::Low => CounterResponse {
                counter_measure: CounterMeasure::Monitor,
                priority: 1,
                reason: "Drone is outside engagement range. Continue monitoring."
                    .to_string(),
            },

            ThreatLevel::Medium => CounterResponse {
                counter_measure: CounterMeasure::RFJammer,
                priority: 2,
                reason: "RF interference is sufficient to disrupt communication."
                    .to_string(),
            },

            ThreatLevel::High => CounterResponse {
                counter_measure: CounterMeasure::LaserWeapon,
                priority: 3,
                reason: "Drone approaching AFV. Directed-energy engagement recommended."
                    .to_string(),
            },

            ThreatLevel::Critical => CounterResponse {
                counter_measure: CounterMeasure::MissileInterceptor,
                priority: 4,
                reason: "Immediate neutralization required."
                    .to_string(),
            },
        }
    }
}

pub fn recommend(threat: &ThreatResult) -> CounterResponse {
    let response = CounterMeasureEngine::recommend(threat);

    println!("\n============== COUNTERMEASURE ==============");
    println!("Drone ID       : {}", threat.drone_id);
    println!("Threat Level   : {:?}", threat.threat_level);
    println!("Threat Score   : {:.2}", threat.threat_score);
    println!("Drone Type     : {:?}", threat.drone_type);
    println!("Confidence     : {:.2}", threat.confidence);
    println!("Distance       : {:.2} m", threat.distance);
    println!("Speed          : {:.2} m/s", threat.speed);

    println!("--------------------------------------------");

    println!("Countermeasure : {:?}", response.counter_measure);
    println!("Priority       : {}", response.priority);
    println!("Reason         : {}", response.reason);

    println!("============================================");

    response
}