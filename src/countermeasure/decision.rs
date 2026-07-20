use crate::models::structs::{
    CounterMeasure,
    Recommendation,
};

#[derive(Debug, Clone)]
pub enum EngagementStatus {
    Standby,
    Monitor,
    Engage,
    Emergency,
}

#[derive(Debug)]
pub struct Decision {
    pub status: EngagementStatus,
    pub execute: bool,
    pub counter_measure: CounterMeasure,
    pub priority: u8,
    pub message: String,
}

pub struct DecisionEngine;

impl DecisionEngine {
    pub fn evaluate(recommendation: &Recommendation) -> Decision {
        match recommendation.counter_measure {
            CounterMeasure::Monitor => Decision {
                status: EngagementStatus::Monitor,
                execute: false,
                counter_measure: CounterMeasure::Monitor,
                priority: recommendation.priority,
                message: "Continue monitoring the target.".to_string(),
            },

            CounterMeasure::RFJammer => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::RFJammer,
                priority: recommendation.priority,
                message: "Activate RF Jammer.".to_string(),
            },

            CounterMeasure::GPSJammer => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::GPSJammer,
                priority: recommendation.priority,
                message: "Activate GPS Jammer.".to_string(),
            },

            CounterMeasure::Spoofing => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::Spoofing,
                priority: recommendation.priority,
                message: "Initiate GPS Spoofing.".to_string(),
            },

            CounterMeasure::LaserWeapon => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::LaserWeapon,
                priority: recommendation.priority,
                message: "Fire Directed Energy Weapon.".to_string(),
            },

            CounterMeasure::MissileInterceptor => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::MissileInterceptor,
                priority: recommendation.priority,
                message: "Launch Missile Interceptor.".to_string(),
            },

            CounterMeasure::CIWS => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::CIWS,
                priority: recommendation.priority,
                message: "Activate CIWS.".to_string(),
            },

            CounterMeasure::SmokeScreen => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::SmokeScreen,
                priority: recommendation.priority,
                message: "Deploy Smoke Screen.".to_string(),
            },

            CounterMeasure::Retreat => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::Retreat,
                priority: recommendation.priority,
                message: "Immediate vehicle retreat.".to_string(),
            },
        }
    }
}