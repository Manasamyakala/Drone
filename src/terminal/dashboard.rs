use crate::models::structs::{
    CounterMeasure,
    DroneType,
    ThreatLevel,
    TrackedDrone,
};

use crate::simulator::afv::AFV;
use crate::terminal::color::Color;

pub struct Dashboard;

impl Dashboard {
    pub fn render(
        afv: &AFV,
        drones: &[TrackedDrone],
    ) {
        Self::clear();
        Self::banner();
        Self::afv_status(afv);
        Self::drone_table(drones);
        Self::footer();
    }

    fn banner() {
        println!(
            "{}",
            Color::border("==============================================================")
        );
        println!(
            "{}",
            Color::title("            AI POWERED COUNTER SYSTEM FOR AFVs")
        );
        println!(
            "{}",
            Color::border("==============================================================")
        );
    }

    fn afv_status(
        afv: &AFV,
    ) {
        println!();

        println!("{}", Color::title("AFV STATUS"));

        println!("Vehicle ID          : {}", afv.vehicle_id);
        println!("Vehicle             : {}", afv.vehicle_name);
        println!("Health              : {:.1}%", afv.health);
        println!("Fuel                : {:.1}%", afv.fuel);
        println!("Ammunition          : {}", afv.ammunition);
        println!("Tracked Drones      : {}", afv.tracked_drones.len());

        println!();
    }

    fn drone_table(
        drones: &[TrackedDrone],
    ) {
        println!("{}", Color::title("ACTIVE DRONES"));

        println!("-------------------------------------------------------------------------------------------------------------");

        println!(
            "{:<6} {:<15} {:<10} {:<10} {:<10} {:<10} {:<12} {:<18}",
            "ID",
            "TYPE",
            "CONF",
            "SPEED",
            "DIST",
            "DIR",
            "THREAT",
            "COUNTER"
        );

        println!("-------------------------------------------------------------------------------------------------------------");

        for drone in drones {
            let threat = calculate_threat(drone);
            let counter = recommend_countermeasure(&threat);

            println!(
                "{:<6} {:<15} {:<10.2} {:<10.2} {:<10.2} {:<10} {:<12} {:<18}",
                drone.drone_id,
                drone_type_to_string(&drone.drone_type),
                drone.confidence,
                drone.speed,
                drone.distance,
                drone.direction,
                threat_to_string(&threat),
                countermeasure_to_string(&counter),
            );
        }

        println!();
    }
        fn footer() {
        println!(
            "{}",
            Color::border("==============================================================")
        );

        println!("{}", Color::info("System Ready"));

        println!(
            "{}",
            Color::border("==============================================================")
        );
    }

    fn clear() {
        print!("\x1B[2J\x1B[1;1H");
    }
}

fn calculate_threat(
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

fn recommend_countermeasure(
    level: &ThreatLevel,
) -> CounterMeasure {
    match *level {
        ThreatLevel::Low => CounterMeasure::Monitor,
        ThreatLevel::Medium => CounterMeasure::RFJammer,
        ThreatLevel::High => CounterMeasure::LaserWeapon,
        ThreatLevel::Critical => CounterMeasure::MissileInterceptor,
    }
}

fn drone_type_to_string(
    drone: &DroneType,
) -> &'static str {
    match drone {
        DroneType::Attack => "Attack",
        DroneType::Surveillance => "Surveillance",
        DroneType::Swarm => "Swarm",
        DroneType::Unknown => "Unknown",
    }
}

fn threat_to_string(
    level: &ThreatLevel,
) -> &'static str {
    match *level {
        ThreatLevel::Low => "LOW",
        ThreatLevel::Medium => "MEDIUM",
        ThreatLevel::High => "HIGH",
        ThreatLevel::Critical => "CRITICAL",
    }
}

fn countermeasure_to_string(
    counter: &CounterMeasure,
) -> &'static str {
    match *counter {
        CounterMeasure::Monitor => "Monitor",
        CounterMeasure::RFJammer => "RF Jammer",
        CounterMeasure::GPSJammer => "GPS Jammer",
        CounterMeasure::Spoofing => "GPS Spoofing",
        CounterMeasure::LaserWeapon => "Laser Weapon",
        CounterMeasure::MissileInterceptor => "Missile",
        CounterMeasure::CIWS => "CIWS",
        CounterMeasure::SmokeScreen => "Smoke Screen",
        CounterMeasure::Retreat => "Retreat",
    }
}