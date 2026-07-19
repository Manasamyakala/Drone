use opencv::{
    core::{Size, INTER_LINEAR},
    imgproc,
    prelude::*,
};

pub struct ImageResizer;

impl ImageResizer {
    /// Resize an image to the desired width and height.
    ///
    /// Example:
    /// let resized = ImageResizer::resize(&image, 640, 640)?;
    pub fn resize(
        image: &Mat,
        width: i32,
        height: i32,
    ) -> opencv::Result<Mat> {

        let mut resized = Mat::default();

        imgproc::resize(
            image,
            &mut resized,
            Size::new(width, height),
            0.0,
            0.0,
            INTER_LINEAR,
        )?;

        Ok(resized)
    }

    /// Resize using the default YOLOv8 input size (640x640).
    pub fn resize_for_yolo(
        image: &Mat,
    ) -> opencv::Result<Mat> {

        Self::resize(image, 640, 640)
    }
}