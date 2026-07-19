use crate::countermeasure::recommender::{
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

    pub fn evaluate(
        recommendation: &Recommendation,
    ) -> Decision {

        match recommendation.counter_measure {

            CounterMeasure::Monitor => Decision {
                status: EngagementStatus::Monitor,
                execute: false,
                counter_measure: CounterMeasure::Monitor,
                priority: recommendation.priority,
                message: String::from(
                    "Continue monitoring the target."
                ),
            },

            CounterMeasure::RFJammer => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::RFJammer,
                priority: recommendation.priority,
                message: String::from(
                    "Activate RF Jammer."
                ),
            },

            CounterMeasure::GPSJammer => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::GPSJammer,
                priority: recommendation.priority,
                message: String::from(
                    "Activate GPS Jammer."
                ),
            },

            CounterMeasure::Spoofing => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::Spoofing,
                priority: recommendation.priority,
                message: String::from(
                    "Initiate GPS Spoofing."
                ),
            },

            CounterMeasure::LaserWeapon => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::LaserWeapon,
                priority: recommendation.priority,
                message: String::from(
                    "Fire Directed Energy Weapon."
                ),
            },

            CounterMeasure::MissileInterceptor => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::MissileInterceptor,
                priority: recommendation.priority,
                message: String::from(
                    "Launch Missile Interceptor."
                ),
            },

            CounterMeasure::CIWS => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::CIWS,
                priority: recommendation.priority,
                message: String::from(
                    "Activate CIWS."
                ),
            },

            CounterMeasure::SmokeScreen => Decision {
                status: EngagementStatus::Engage,
                execute: true,
                counter_measure: CounterMeasure::SmokeScreen,
                priority: recommendation.priority,
                message: String::from(
                    "Deploy Smoke Screen."
                ),
            },

            CounterMeasure::Retreat => Decision {
                status: EngagementStatus::Emergency,
                execute: true,
                counter_measure: CounterMeasure::Retreat,
                priority: recommendation.priority,
                message: String::from(
                    "Immediate vehicle retreat."
                ),
            },
        }
    }
}