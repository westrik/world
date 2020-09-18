// use image::imageops::{resize, FilterType};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::collections::HashMap;

pub fn constrain_image_dimensions(
    image: DynamicImage,
    widths: Vec<u32>,
) -> HashMap<u32, DynamicImage> {
    let results: Vec<(u32, DynamicImage)> = widths
        .into_iter()
        .filter_map(|new_width| {
            let dimensions = image.dimensions();
            if dimensions.0 <= new_width {
                None
            } else {
                let scaling_factor = new_width as f32 / dimensions.0 as f32;
                let new_height = dimensions.1 as f32 * scaling_factor;
                let resized =
                    image.resize(new_width, new_height.ceil() as u32, FilterType::CatmullRom);
                Some((new_width, resized))
            }
        })
        .collect();
    results.into_iter().collect()
}

// TODO: pub fn content_based_crop()

#[cfg(test)]
pub mod resize_image {

    // TODO: test images w/ various properties
    // - animated gif
    // - non-animated gif
    // - png
    // - jpeg
    // - tiff
    // - webp
    // - 10-bit color (where applicable)

    use super::*;
    use image::DynamicImage;

    #[test]
    fn test_constrain_image_dimensions() {
        let widths = vec![360, 720, 1200, 2200];
        let test_image = DynamicImage::new_rgba8(2000, 1000);
        let constrained = constrain_image_dimensions(test_image, widths);
        println!("{:#?}", constrained.keys());
    }
}
