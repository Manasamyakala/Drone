use ndarray::Array4;
use ort::{
    inputs,
    session::Session,
    value::TensorRef,
};
use std::error::Error;
use std::path::Path;

pub struct OnnxDetector {
    session: Session,
}

impl OnnxDetector {
    const MODEL_PATH: &'static str = "datasets/models/yolov8n.onnx";

    pub fn new() -> Result<Self, Box<dyn Error>> {
        println!("========================================");
        println!("Loading YOLOv8 ONNX Model...");
        println!("Model Path: {}", Self::MODEL_PATH);
        println!("========================================");

        if !Path::new(Self::MODEL_PATH).exists() {
            return Err(format!(
                "Model not found!\nExpected location:\n{}",
                Self::MODEL_PATH
            )
            .into());
        }

        let session = Session::builder()?
            .commit_from_file(Self::MODEL_PATH)?;

        println!("Model loaded successfully.\n");

        Ok(Self { session })
    }

    pub fn from_path(model_path: &str) -> Result<Self, Box<dyn Error>> {
        if !Path::new(model_path).exists() {
            return Err(format!("Model file not found: {}", model_path).into());
        }

        let session = Session::builder()?
            .commit_from_file(model_path)?;

        Ok(Self { session })
    }

    pub fn infer(
        &mut self,
        input: Array4<f32>,
    ) -> Result<Vec<f32>, Box<dyn Error>> {

        let tensor = TensorRef::from_array_view(input.view())?;

        let outputs = self.session.run(inputs![tensor])?;

        let (_, output) = outputs[0]
            .try_extract_tensor::<f32>()?;

        Ok(output.to_vec())
    }
}