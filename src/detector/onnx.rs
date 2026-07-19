use ndarray::Array4;
use ort::{
    session::Session,
    value::TensorRef,
};
use std::error::Error;
use std::path::Path;

pub struct OnnxDetector {
    session: Session,
}

impl OnnxDetector {
    /// Load the YOLO ONNX model
    pub fn new(model_path: &str) -> Result<Self, Box<dyn Error>> {

        let session = Session::builder()?
            .commit_from_file(Path::new(model_path))?;

        Ok(Self {
            session,
        })
    }

    /// Run inference on a preprocessed tensor
    pub fn infer(
        &mut self,
        input: Array4<f32>,
    ) -> Result<Vec<f32>, Box<dyn Error>> {

        let tensor = TensorRef::from_array_view(input.view())?;

        let outputs = self.session.run(ort::inputs![
            tensor
        ])?;

        // First output tensor
        let output = outputs[0]
            .try_extract_tensor::<f32>()?;

        let predictions = output
            .iter()
            .copied()
            .collect::<Vec<f32>>();

        Ok(predictions)
    }
}