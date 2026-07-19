use crate::models::structs::{
    BoundingBox,
    DroneType,
};

#[derive(Debug, Clone)]
pub enum DroneStatus {
    Detected,
    Tracking,
    Threat,
    Neutralized,
    Lost,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Clone)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
    pub vz: f32,
}

#[derive(Debug, Clone)]
pub struct Drone {

    /// Unique Drone ID
    pub id: u32,

    /// Drone Category
    pub drone_type: DroneType,

    /// Detection Confidence
    pub confidence: f32,

    /// Bounding Box
    pub bbox: BoundingBox,

    /// Current Position
    pub position: Position,

    /// Velocity Vector
    pub velocity: Velocity,

    /// Estimated Speed (m/s)
    pub speed: f32,

    /// Estimated Distance from AFV (meters)
    pub distance: f32,

    /// Flying Direction
    pub direction: String,

    /// Current Status
    pub status: DroneStatus,

    /// Track History
    pub trajectory: Vec<Position>,
}

impl Drone {

    pub fn new(
        id: u32,
        drone_type: DroneType,
        confidence: f32,
        bbox: BoundingBox,
    ) -> Self {

        Self {

            id,

            drone_type,

            confidence,

            bbox,

            position: Position {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },

            velocity: Velocity {
                vx: 0.0,
                vy: 0.0,
                vz: 0.0,
            },

            speed: 0.0,

            distance: 0.0,

            direction: String::from("Unknown"),

            status: DroneStatus::Detected,

            trajectory: Vec::new(),
        }
    }

    /// Update drone position
    pub fn update_position(
        &mut self,
        x: f32,
        y: f32,
        z: f32,
    ) {

        self.position.x = x;
        self.position.y = y;
        self.position.z = z;

        self.trajectory.push(self.position.clone());
    }

    /// Update velocity
    pub fn update_velocity(
        &mut self,
        vx: f32,
        vy: f32,
        vz: f32,
    ) {

        self.velocity.vx = vx;
        self.velocity.vy = vy;
        self.velocity.vz = vz;
    }

    /// Update speed
    pub fn update_speed(
        &mut self,
        speed: f32,
    ) {

        self.speed = speed;
    }

    /// Update distance
    pub fn update_distance(
        &mut self,
        distance: f32,
    ) {

        self.distance = distance;
    }

    /// Update direction
    pub fn update_direction(
        &mut self,
        direction: String,
    ) {

        self.direction = direction;
    }

    /// Update drone status
    pub fn update_status(
        &mut self,
        status: DroneStatus,
    ) {

        self.status = status;
    }

    /// Check if drone is dangerous
    pub fn is_threat(
        &self,
    ) -> bool {

        self.distance < 150.0 && self.speed > 10.0
    }

    /// Distance from AFV
    pub fn distance_from(
        &self,
        afv_x: f32,
        afv_y: f32,
        afv_z: f32,
    ) -> f32 {

        let dx = self.position.x - afv_x;
        let dy = self.position.y - afv_y;
        let dz = self.position.z - afv_z;

        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}