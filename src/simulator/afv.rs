use crate::models::structs::{
    CounterMeasure,
    TrackedDrone,
};

#[derive(Debug, Clone)]
pub enum AFVStatus {
    Idle,
    Monitoring,
    Tracking,
    Engaging,
    UnderAttack,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone)]
pub struct AFV {

    /// Vehicle ID
    pub vehicle_id: String,

    /// Vehicle Name
    pub vehicle_name: String,

    /// Current GPS Position
    pub position: Position,

    /// Health Percentage
    pub health: f32,

    /// Ammunition Remaining
    pub ammunition: u32,

    /// Fuel Percentage
    pub fuel: f32,

    /// Current Status
    pub status: AFVStatus,

    /// Active Countermeasure
    pub active_countermeasure: Option<CounterMeasure>,

    /// Tracked Drones
    pub tracked_drones: Vec<TrackedDrone>,
}

impl AFV {

    pub fn new() -> Self {

        Self {

            vehicle_id: String::from("AFV-001"),

            vehicle_name: String::from("Main Battle Tank"),

            position: Position {

                latitude: 0.0,

                longitude: 0.0,
            },

            health: 100.0,

            ammunition: 100,

            fuel: 100.0,

            status: AFVStatus::Idle,

            active_countermeasure: None,

            tracked_drones: Vec::new(),
        }
    }

    /// Update AFV Position
    pub fn update_position(
        &mut self,
        latitude: f64,
        longitude: f64,
    ) {

        self.position.latitude = latitude;
        self.position.longitude = longitude;
    }

    /// Add Newly Tracked Drone
    pub fn add_drone(
        &mut self,
        drone: TrackedDrone,
    ) {

        self.tracked_drones.push(drone);

        self.status = AFVStatus::Tracking;
    }

    /// Clear Previous Frame Drones
    pub fn clear_tracked_drones(
        &mut self,
    ) {

        self.tracked_drones.clear();
    }

    /// Activate Countermeasure
    pub fn activate_countermeasure(
        &mut self,
        counter: CounterMeasure,
    ) {

        self.active_countermeasure = Some(counter);

        self.status = AFVStatus::Engaging;
    }

    /// Vehicle Damage
    pub fn receive_damage(
        &mut self,
        damage: f32,
    ) {

        if damage >= self.health {

            self.health = 0.0;

        } else {

            self.health -= damage;
        }

        self.status = AFVStatus::UnderAttack;
    }

    /// Consume Fuel
    pub fn consume_fuel(
        &mut self,
        amount: f32,
    ) {

        if amount >= self.fuel {

            self.fuel = 0.0;

        } else {

            self.fuel -= amount;
        }
    }

    /// Consume Ammunition
    pub fn consume_ammunition(
        &mut self,
        amount: u32,
    ) {

        if amount >= self.ammunition {

            self.ammunition = 0;

        } else {

            self.ammunition -= amount;
        }
    }

    /// Number of Active Drones
    pub fn active_targets(&self) -> usize {

        self.tracked_drones.len()
    }
}