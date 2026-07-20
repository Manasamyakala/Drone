use image::{imageops::FilterType, DynamicImage};

pub struct ImageResizer;

impl ImageResizer {
    pub fn resize(
        image: DynamicImage,
        width: u32,
        height: u32,
    ) -> DynamicImage {
        image.resize_exact(width, height, FilterType::CatmullRom)
    }
}