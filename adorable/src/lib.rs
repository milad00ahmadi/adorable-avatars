use std::io::Cursor;
use std::sync::RwLockReadGuard;

use image::io::Reader as ImageReader;
use image::{imageops, DynamicImage, ImageBuffer, ImageOutputFormat};

pub use face::create_face_factory;

pub use crate::face::FaceFactory;

mod face;
mod utils;

pub fn create_avatar(
    face_factory: RwLockReadGuard<FaceFactory>,
    size: u32,
    input: &str,
) -> Vec<u8> {
    let face = face_factory.create(input);
    let mut background = ImageBuffer::from_pixel(400, 400, face.color);

    imageops::overlay(&mut background, &face.eyes, 0, 0);
    imageops::overlay(&mut background, &face.mouth, 0, 0);
    imageops::overlay(&mut background, &face.nose, 0, 0);
    let background = imageops::resize(&background, size, size, imageops::FilterType::Triangle);

    let final_image = DynamicImage::ImageRgba8(background);
    let mut bytes: Vec<u8> = Vec::with_capacity(500);
    final_image
        .write_to(&mut Cursor::new(&mut bytes), ImageOutputFormat::Png)
        .unwrap();

    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face() {}

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
