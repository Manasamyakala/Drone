use crate::models::structs::TrackedDrone;
use crate::simulator::afv::AFV;

#[derive(Debug, Clone)]
pub enum Weather {
    Clear,
    Cloudy,
    Rain,
    Fog,
    Storm,
}

#[derive(Debug, Clone)]
pub enum MissionStatus {
    Idle,
    Surveillance,
    ThreatDetected,
    Engagement,
    MissionComplete,
}

#[derive(Debug, Clone)]
pub struct Battlefield {

    /// Battlefield Name
    pub name: String,

    /// Width (meters)
    pub width: f32,

    /// Height (meters)
    pub height: f32,

    /// Weather Condition
    pub weather: Weather,

    /// Visibility (meters)
    pub visibility: f32,

    /// Mission Status
    pub mission_status: MissionStatus,

    /// AFVs in Battlefield
    pub afvs: Vec<AFV>,

    /// Enemy Drones
    pub drones: Vec<TrackedDrone>,
}

impl Battlefield {

    /// Create Battlefield
    pub fn new(
        name: String,
        width: f32,
        height: f32,
    ) -> Self {

        Self {

            name,

            width,

            height,

            weather: Weather::Clear,

            visibility: 5000.0,

            mission_status: MissionStatus::Idle,

            afvs: Vec::new(),

            drones: Vec::new(),
        }
    }

    /// Add AFV
    pub fn add_afv(
        &mut self,
        afv: AFV,
    ) {

        self.afvs.push(afv);
    }

    /// Add Detected Drone
    pub fn add_drone(
        &mut self,
        drone: TrackedDrone,
    ) {

        self.drones.push(drone);

        self.mission_status =
            MissionStatus::ThreatDetected;
    }

    /// Remove All Drones
    pub fn clear_drones(
        &mut self,
    ) {

        self.drones.clear();

        self.mission_status =
            MissionStatus::Idle;
    }

    /// Set Weather
    pub fn update_weather(
        &mut self,
        weather: Weather,
    ) {

        self.weather = weather;
    }

    /// Update Visibility
    pub fn update_visibility(
        &mut self,
        visibility: f32,
    ) {

        self.visibility = visibility;
    }

    /// Total AFVs
    pub fn total_afvs(
        &self,
    ) -> usize {

        self.afvs.len()
    }

    /// Total Active Drones
    pub fn total_drones(
        &self,
    ) -> usize {

        self.drones.len()
    }

    /// Mission Summary
    pub fn summary(
        &self,
    ) {

        println!("\n========== Battlefield ==========");

        println!("Name          : {}", self.name);

        println!("Area          : {} x {} meters",
            self.width,
            self.height
        );

        println!("Weather       : {:?}", self.weather);

        println!("Visibility    : {} m",
            self.visibility
        );

        println!("Mission       : {:?}",
            self.mission_status
        );

        println!("AFVs          : {}",
            self.total_afvs()
        );

        println!("Enemy Drones  : {}",
            self.total_drones()
        );

        println!("=================================\n");
    }
}