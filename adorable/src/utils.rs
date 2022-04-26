use std::fs;
use std::io::Cursor;

use bytes::Bytes;
use image::{DynamicImage, Rgba};

use crate::ImageReader;

pub(crate) fn get_eyes() -> Vec<DynamicImage> {
    vec![
        Bytes::from(fs::read("./adorable/img/eyes/eyes1.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes2.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes3.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes4.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes5.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes6.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes7.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes9.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/eyes/eyes10.png").unwrap()),
    ]
    .into_iter()
    .map(|value| {
        ImageReader::new(Cursor::new(value))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
    })
    .collect()
}

pub(crate) fn get_mouths() -> Vec<DynamicImage> {
    vec![
        Bytes::from(fs::read("./adorable/img/mouths/mouth1.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/mouths/mouth3.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/mouths/mouth5.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/mouths/mouth6.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/mouths/mouth7.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/mouths/mouth9.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/mouths/mouth10.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/mouths/mouth11.png").unwrap()),
    ]
    .into_iter()
    .map(|value| {
        ImageReader::new(Cursor::new(value))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
    })
    .collect()
}

pub(crate) fn get_noses() -> Vec<DynamicImage> {
    vec![
        Bytes::from(fs::read("./adorable/img/noses/nose2.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/noses/nose3.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/noses/nose4.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/noses/nose5.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/noses/nose6.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/noses/nose7.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/noses/nose8.png").unwrap()),
        Bytes::from(fs::read("./adorable/img/noses/nose9.png").unwrap()),
    ]
    .into_iter()
    .map(|value| {
        ImageReader::new(Cursor::new(value))
            .with_guessed_format()
            .unwrap()
            .decode()
            .unwrap()
    })
    .collect()
}

pub(crate) fn get_colors() -> Vec<Rgba<u8>> {
    vec![
        Rgba::from([129, 190, 240, 255]),
        Rgba::from([173, 139, 242, 255]),
        Rgba::from([191, 242, 136, 255]),
        Rgba::from([222, 120, 120, 255]),
        Rgba::from([165, 170, 197, 255]),
        Rgba::from([111, 242, 197, 255]),
        Rgba::from([240, 218, 94, 255]),
        Rgba::from([235, 89, 114, 255]),
        Rgba::from([246, 190, 93, 255]),
    ]
}
