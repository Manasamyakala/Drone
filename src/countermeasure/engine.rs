use crate::threat_engine::analyzer::{ThreatLevel, ThreatResult};

#[derive(Debug, Clone)]
pub enum CounterMeasure {
    Monitor,
    RFJammer,
    GPSJammer,
    LaserWeapon,
    MissileInterceptor,
    SmokeScreen,
    CIWS,
    Retreat,
}

#[derive(Debug)]
pub struct CounterResponse {
    pub counter_measure: CounterMeasure,
    pub priority: u8,
    pub reason: String,
}

pub struct CounterMeasureEngine;

impl CounterMeasureEngine {
    pub fn recommend(threat: &ThreatResult) -> CounterResponse {

        match threat.level {

            ThreatLevel::Low => CounterResponse {
                counter_measure: CounterMeasure::Monitor,
                priority: 1,
                reason: String::from(
                    "Drone is outside engagement range. Continue monitoring."
                ),
            },

            ThreatLevel::Medium => CounterResponse {
                counter_measure: CounterMeasure::RFJammer,
                priority: 2,
                reason: String::from(
                    "RF interference is sufficient to disrupt communication."
                ),
            },

            ThreatLevel::High => CounterResponse {
                counter_measure: CounterMeasure::LaserWeapon,
                priority: 3,
                reason: String::from(
                    "Drone approaching AFV. Directed-energy engagement recommended."
                ),
            },

            ThreatLevel::Critical => CounterResponse {
                counter_measure: CounterMeasure::MissileInterceptor,
                priority: 4,
                reason: String::from(
                    "Immediate neutralization required."
                ),
            },
        }
    }
}

pub fn recommend(threat: &ThreatResult) -> CounterResponse {

    let response = CounterMeasureEngine::recommend(threat);

    println!("\n============== COUNTERMEASURE ==============");

    println!("Threat Level   : {:?}", threat.level);
    println!("Threat Score   : {:.2}", threat.score);

    println!("Countermeasure : {:?}", response.counter_measure);

    println!("Priority       : {}", response.priority);

    println!("Reason         : {}", response.reason);

    println!("============================================");

    response
}