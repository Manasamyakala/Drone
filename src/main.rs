mod config;
mod loader;
mod preprocessing;
mod detector;
// mod classifier; // Uncomment after creating the classifier module
mod tracker;
mod trajectory;
mod simulator;
mod threat_engine;
mod countermeasure;
mod terminal;
mod database;
mod models;
mod utils;

// use terminal::dashboard::show_banner;

fn main() {
    show_banner();

    println!("\n==================================================");
    println!("Initializing AI Counter System...");
    println!("==================================================");

    config::settings::load();

    println!("\n[1] Loading Dataset...");
    loader::dataset_loader::load_dataset();

    println!("\n[2] Preprocessing Frames...");
    preprocessing::image_processor::preprocess();

    println!("\n[3] Detecting Drones...");
    detector::yolo_detector::detect();

    println!("\n[4] Classifying Drone...");
    classifier::drone_classifier::classify();

    println!("\n[5] Tracking Drone...");
    tracker::multi_tracker::track();

    println!("\n[6] Predicting Trajectory...");
    trajectory::predictor::predict();

    println!("\n[7] Simulating Drone Parameters...");
    simulator::drone_simulator::simulate();

    println!("\n[8] Assessing Threat...");
    threat_engine::analyzer::analyze();

    println!("\n[9] Recommending Countermeasure...");
    countermeasure::engine::recommend();

    println!("\n[10] Launching Terminal Dashboard...");
    terminal::dashboard::start();

    println!("\nMission Completed Successfully.");
}