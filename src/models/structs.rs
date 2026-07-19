use serde::{Deserialize, Serialize};

/// Bounding Box returned by YOLO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

/// Drone Category
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DroneType {
    Surveillance,
    Attack,
    Swarm,
    Unknown,
}

/// Threat Levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Available Counter Measures
#[derive(Debug, Clone, Serialize, Deserialize)]
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

/// YOLO Detection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Detection {

    pub id: Option<u32>,

    pub class_id: usize,

    pub class_name: String,

    pub confidence: f32,

    pub bbox: BoundingBox,
}

/// Tracked Drone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackedDrone {

    pub drone_id: u32,

    pub drone_type: DroneType,

    pub confidence: f32,

    pub bbox: BoundingBox,

    pub center_x: f32,

    pub center_y: f32,

    pub speed: f32,

    pub distance: f32,

    pub direction: String,

    pub trajectory: Vec<(f32, f32)>,
}

/// Threat Analysis Result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatResult {

    pub drone_id: u32,

    pub drone_type: DroneType,

    pub threat_level: ThreatLevel,

    pub threat_score: f32,

    pub confidence: f32,

    pub distance: f32,

    pub speed: f32,
}

/// Countermeasure Recommendation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {

    pub drone_id: u32,

    pub counter_measure: CounterMeasure,

    pub priority: u8,

    pub confidence: f32,

    pub reason: String,
}

/// Final Engagement Decision
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Decision {

    pub execute: bool,

    pub message: String,

    pub recommendation: Recommendation,
}

/// Dashboard Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardRecord {

    pub timestamp: String,

    pub tracked_drone: TrackedDrone,

    pub threat: ThreatResult,

    pub recommendation: Recommendation,
}

/// Database Record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseRecord {

    pub id: i64,

    pub timestamp: String,

    pub drone_id: u32,

    pub drone_type: DroneType,

    pub confidence: f32,

    pub speed: f32,

    pub distance: f32,

    pub direction: String,

    pub threat_level: ThreatLevel,

    pub threat_score: f32,

    pub counter_measure: CounterMeasure,
}