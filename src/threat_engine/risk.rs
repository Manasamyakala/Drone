use crate::models::structs::{ThreatLevel, ThreatResult};

#[derive(Debug, Clone)]
pub enum RiskCategory {
    Safe,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone)]
pub struct RiskAssessment {
    pub drone_id: u32,
    pub threat_level: ThreatLevel,
    pub threat_score: f32,
    pub risk_percentage: f32,
    pub risk_category: RiskCategory,
    pub description: String,
}

pub fn assess(result: &ThreatResult) -> RiskAssessment {
    let percentage = result.threat_score.clamp(0.0, 100.0);

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
        RiskCategory::Safe => "No immediate threat detected.",
        RiskCategory::Low => "Drone activity should be monitored.",
        RiskCategory::Medium => "Potential threat. Prepare defensive systems.",
        RiskCategory::High => "High-risk drone. Countermeasures recommended.",
        RiskCategory::Critical => "Critical threat. Immediate engagement required.",
    };

    RiskAssessment {
        drone_id: result.drone_id,
        threat_level: result.threat_level.clone(),
        threat_score: result.threat_score,
        risk_percentage: percentage,
        risk_category: category,
        description: description.to_string(),
    }
}