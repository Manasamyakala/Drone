use ndarray::Array4;
use opencv::{
    core,
    imgproc,
    prelude::*,
};

pub struct ImageNormalizer;

impl ImageNormalizer {
    /// Convert OpenCV Mat into a normalized tensor
    /// Output Shape: [1, 3, H, W]
    pub fn normalize(image: &Mat) -> opencv::Result<Array4<f32>> {

        // Convert BGR -> RGB
        let mut rgb = Mat::default();

        imgproc::cvt_color(
            image,
            &mut rgb,
            imgproc::COLOR_BGR2RGB,
            0,
        )?;

        let rows = rgb.rows() as usize;
        let cols = rgb.cols() as usize;

        // NCHW Tensor
        let mut tensor =
            Array4::<f32>::zeros((1, 3, rows, cols));

        for y in 0..rows {

            for x in 0..cols {

                let pixel =
                    *rgb.at_2d::<core::Vec3b>(
                        y as i32,
                        x as i32,
                    )?;

                // Normalize to [0,1]
                tensor[[0, 0, y, x]] =
                    pixel[0] as f32 / 255.0;

                tensor[[0, 1, y, x]] =
                    pixel[1] as f32 / 255.0;

                tensor[[0, 2, y, x]] =
                    pixel[2] as f32 / 255.0;
            }
        }

        Ok(tensor)
    }
}