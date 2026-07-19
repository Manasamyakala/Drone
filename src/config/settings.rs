use std::path::Path;

pub struct Config {
    pub project_name: String,
    pub dataset_path: String,
    pub model_path: String,
    pub output_path: String,
    pub log_path: String,
    pub confidence_threshold: f32,
    pub nms_threshold: f32,
    pub max_drones: u32,
    pub enable_tracking: bool,
    pub enable_prediction: bool,
    pub enable_logging: bool,
}

impl Config {
    /// Default configuration
    pub fn new() -> Self {
        Self {
            project_name: String::from("AI Powered Counter System for AFVs"),

            dataset_path: String::from("datasets/"),

            model_path: String::from("models/yolov8.onnx"),

            output_path: String::from("output/"),

            log_path: String::from("logs/"),

            confidence_threshold: 0.50,

            nms_threshold: 0.45,

            max_drones: 25,

            enable_tracking: true,

            enable_prediction: true,

            enable_logging: true,
        }
    }

    /// Display configuration
    pub fn display(&self) {
        println!("-------------------------------------------------------");
        println!("SYSTEM CONFIGURATION");
        println!("-------------------------------------------------------");

        println!("Project Name        : {}", self.project_name);
        println!("Dataset Path        : {}", self.dataset_path);
        println!("Model Path          : {}", self.model_path);
        println!("Output Path         : {}", self.output_path);
        println!("Log Path            : {}", self.log_path);
        println!("Confidence Threshold: {}", self.confidence_threshold);
        println!("NMS Threshold       : {}", self.nms_threshold);
        println!("Maximum Drones      : {}", self.max_drones);
        println!("Tracking Enabled    : {}", self.enable_tracking);
        println!("Prediction Enabled  : {}", self.enable_prediction);
        println!("Logging Enabled     : {}", self.enable_logging);

        println!("-------------------------------------------------------");
    }

    /// Verify required folders/files
    pub fn verify(&self) {
        println!("\nChecking Project Resources...");

        if Path::new(&self.dataset_path).exists() {
            println!("✓ Dataset Folder Found");
        } else {
            println!("✗ Dataset Folder Missing");
        }

        if Path::new(&self.model_path).exists() {
            println!("✓ YOLO Model Found");
        } else {
            println!("✗ YOLO Model Missing");
        }

        if Path::new(&self.output_path).exists() {
            println!("✓ Output Folder Found");
        } else {
            println!("✗ Output Folder Missing");
        }

        if Path::new(&self.log_path).exists() {
            println!("✓ Log Folder Found");
        } else {
            println!("✗ Log Folder Missing");
        }

        println!("Resource Check Completed.\n");
    }
}

/// Called from main.rs
pub fn load() {
    println!("Loading Configuration...");

    let config = Config::new();

    config.display();

    config.verify();

    println!("Configuration Loaded Successfully.\n");
}