use crate::models::structs::ThreatLevel;
use crate::threat_engine::analyzer::ThreatResult;

#[derive(Debug, Clone)]
pub struct RiskAssessment {
    /// Drone ID
    pub drone_id: u32,

    /// Threat level assigned by analyzer
    pub threat_level: ThreatLevel,

    /// Numerical threat score
    pub threat_score: f32,

    /// Risk percentage (0-100)
    pub risk_percentage: f32,

    /// Risk category
    pub risk_category: RiskCategory,

    /// Explanation
    pub description: String,
}

#[derive(Debug, Clone)]
pub enum RiskCategory {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

pub struct RiskEngine;

impl RiskEngine {
    /// Assess the risk for a single drone
    pub fn assess(result: &ThreatResult) -> RiskAssessment {
        let percentage = result.score.clamp(0.0, 100.0);

        let category = if percentage >= 90.0 {
            RiskCategory::Critical
        } else if percentage >= 70.0 {
            RiskCategory::High
        } else if percentage >= 40.0 {
            RiskCategory::Medium
        } else if percentage >= 15.0 {
            RiskCategory::Low
        } else {
            RiskCategory::Safe
        };

        let description = match category {
            RiskCategory::Safe =>
                "No immediate threat detected.",

            RiskCategory::Low =>
                "Drone activity should be monitored.",

            RiskCategory::Medium =>
                "Potential threat. Prepare defensive systems.",

            RiskCategory::High =>
                "High-risk drone. Countermeasures recommended.",

            RiskCategory::Critical =>
                "Critical threat. Immediate engagement required.",
        };

        RiskAssessment {
            drone_id: result.drone_id,
            threat_level: result.threat_level.clone(),
            threat_score: result.score,
            risk_percentage: percentage,
            risk_category: category,
            description: description.to_string(),
        }
    }

    /// Assess all analyzed drones
    pub fn assess_all(
        results: &[ThreatResult],
    ) -> Vec<RiskAssessment> {
        results
            .iter()
            .map(Self::assess)
            .collect()
    }

    /// Determine the highest battlefield risk
    pub fn highest_risk(
        assessments: &[RiskAssessment],
    ) -> Option<&RiskAssessment> {
        assessments.iter().max_by(|a, b| {
            a.risk_percentage
                .partial_cmp(&b.risk_percentage)
                .unwrap()
        })
    }

    /// Calculate average battlefield risk
    pub fn average_risk(
        assessments: &[RiskAssessment],
    ) -> f32 {
        if assessments.is_empty() {
            return 0.0;
        }

        let total: f32 = assessments
            .iter()
            .map(|r| r.risk_percentage)
            .sum();

        total / assessments.len() as f32
    }
}