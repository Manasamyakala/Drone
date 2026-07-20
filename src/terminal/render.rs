use crate::models::structs::{
    CounterMeasure,
    DroneType,
    ThreatLevel,
    TrackedDrone,
};

use crate::simulator::afv::AFV;
use crate::terminal::color::Color;

pub struct Render;

impl Render {

    pub fn draw(
        afv: &AFV,
        drones: &[TrackedDrone],
    ) {

        Self::clear();

        Self::draw_banner();

        Self::draw_afv(afv);

        Self::draw_drones(drones);

        Self::draw_summary(afv, drones);
    }

    fn clear() {

        print!("\x1B[2J\x1B[1;1H");
    }

    fn draw_banner() {

        println!("{}", Color::border("================================================================================================================"));

        println!("{}", Color::title("              AI POWERED COUNTER SYSTEM FOR MILITARY VEHICLES (AFVs)"));

        println!("{}", Color::border("================================================================================================================"));
    }

    fn draw_afv(
        afv: &AFV,
    ) {

        println!();

        println!("{}", Color::title("AFV STATUS"));

        println!("Vehicle ID      : {}", afv.vehicle_id);

        println!("Vehicle Name    : {}", afv.vehicle_name);

        println!("Health          : {:.1}%", afv.health);

        println!("Fuel            : {:.1}%", afv.fuel);

        println!("Ammo            : {}", afv.ammunition);

        println!("Status          : {:?}", afv.status);

        println!();
    }

    fn draw_drones(
        drones: &[TrackedDrone],
    ) {

        println!("{}", Color::title("ACTIVE DRONES"));

        println!("--------------------------------------------------------------------------------------------------------------------------");

        println!(
            "{:<6} {:<15} {:<10} {:<10} {:<10} {:<12} {:<12}",
            "ID",
            "TYPE",
            "CONF",
            "SPEED",
            "DIST",
            "THREAT",
            "COUNTER"
        );

        println!("--------------------------------------------------------------------------------------------------------------------------");

        for drone in drones {

            let threat = threat(drone);

            let counter = counter(&threat);

            println!(
                "{:<6} {:<15} {:<10.2} {:<10.2} {:<10.2} {:<12} {:<12}",

                drone.drone_id,

                drone_type(&drone.drone_type),

                drone.confidence,

                drone.speed,

                drone.distance,

                threat_name(&threat),

                counter_name(&counter),
            );
        }

        println!();
    }

    fn draw_summary(
        afv: &AFV,
        drones: &[TrackedDrone],
    ) {

        println!("{}", Color::border("================================================================================================================"));

        println!("Tracked Drones : {}", drones.len());

        println!("Vehicle Health : {:.1}%", afv.health);

        println!("Fuel Remaining : {:.1}%", afv.fuel);

        println!("Ammunition     : {}", afv.ammunition);

        println!("{}", Color::border("================================================================================================================"));
    }
}

fn drone_type(
    drone: &DroneType,
) -> &'static str {

    match drone {

        DroneType::Attack => "Attack",

        DroneType::Surveillance => "Surveillance",

        DroneType::Swarm => "Swarm",

        DroneType::Unknown => "Unknown",
    }
}

fn threat(
    drone: &TrackedDrone,
) -> ThreatLevel {

    if drone.distance < 50.0 {

        ThreatLevel::Critical

    } else if drone.distance < 150.0 {

        ThreatLevel::High

    } else if drone.distance < 300.0 {

        ThreatLevel::Medium

    } else {

        ThreatLevel::Low
    }
}

fn counter(
    level: &ThreatLevel,
) -> CounterMeasure {
    match *level {
        ThreatLevel::Low => CounterMeasure::Monitor,
        ThreatLevel::Medium => CounterMeasure::RFJammer,
        ThreatLevel::High => CounterMeasure::LaserWeapon,
        ThreatLevel::Critical => CounterMeasure::MissileInterceptor,
    }
}

fn threat_name(
    level: &ThreatLevel,
) -> &'static str {
    match *level {
        ThreatLevel::Low => "LOW",
        ThreatLevel::Medium => "MEDIUM",
        ThreatLevel::High => "HIGH",
        ThreatLevel::Critical => "CRITICAL",
    }
}

fn counter_name(
    counter: &CounterMeasure,
) -> &'static str {
    match *counter {
        CounterMeasure::Monitor => "Monitor",
        CounterMeasure::RFJammer => "RF Jammer",
        CounterMeasure::GPSJammer => "GPS Jammer",
        CounterMeasure::Spoofing => "Spoofing",
        CounterMeasure::LaserWeapon => "Laser",
        CounterMeasure::MissileInterceptor => "Missile",
        CounterMeasure::CIWS => "CIWS",
        CounterMeasure::SmokeScreen => "Smoke",
        CounterMeasure::Retreat => "Retreat",
    }
}