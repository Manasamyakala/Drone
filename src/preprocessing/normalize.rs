use image::DynamicImage;
use ndarray::Array4;

pub struct ImageNormalizer;

impl ImageNormalizer {
    pub fn normalize(image: &DynamicImage) -> Array4<f32> {
        let rgb = image.to_rgb8();

        let (width, height) = rgb.dimensions();

        let mut tensor =
            Array4::<f32>::zeros((1, 3, height as usize, width as usize));

        for (x, y, pixel) in rgb.enumerate_pixels() {
            tensor[[0, 0, y as usize, x as usize]] = pixel[0] as f32 / 255.0;
            tensor[[0, 1, y as usize, x as usize]] = pixel[1] as f32 / 255.0;
            tensor[[0, 2, y as usize, x as usize]] = pixel[2] as f32 / 255.0;
        }

        tensor
    }
}