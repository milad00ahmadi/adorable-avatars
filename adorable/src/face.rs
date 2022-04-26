use image::{DynamicImage, Rgba};

use common::hash::hash_factory;
use common::reducers::{sum, sum_and_diff};
use common::Hash;

use crate::utils;

#[derive(Debug, Clone)]
pub struct Face<T> {
    pub color: Rgba<u8>,
    pub nose: T,
    pub mouth: T,
    pub eyes: T,
}

impl<T> Face<T> {
    pub fn new(color: Rgba<u8>, nose: T, mouth: T, eyes: T) -> Self {
        Self {
            color,
            nose,
            mouth,
            eyes,
        }
    }
}

pub struct FaceFactory {
    color_hash: Hash<Rgba<u8>>,
    eye_hash: Hash<DynamicImage>,
    mouth_hash: Hash<DynamicImage>,
    nose_hash: Hash<DynamicImage>,
}

impl FaceFactory {
    pub fn create(&self, value: &str) -> Face<DynamicImage> {
        return Face::new(
            self.color_hash.get(&value).to_owned(),
            self.nose_hash.get(&value).to_owned(),
            self.mouth_hash.get(&value).to_owned(),
            self.eye_hash.get(&value).to_owned(),
        );
    }
}

pub fn create_face_factory() -> FaceFactory {
    let eyes = utils::get_eyes();
    let mouths = utils::get_mouths();
    let noses = utils::get_noses();
    let colors = utils::get_colors();

    return FaceFactory {
        eye_hash: Hash::new(eyes.into(), hash_factory(sum)),
        nose_hash: Hash::new(noses.into(), hash_factory(sum)),
        mouth_hash: Hash::new(mouths.into(), hash_factory(sum_and_diff)),
        color_hash: Hash::new(colors.into(), hash_factory(sum)),
    };
}
