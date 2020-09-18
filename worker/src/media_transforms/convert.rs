use image::{DynamicImage, ImageFormat};
use std::io::Read;

pub fn output_to_png(image: &DynamicImage) -> Result<Vec<u8>, String> {
    let mut file = Vec::new();
    image
        .write_to(&mut file, ImageFormat::Png)
        .map_err(|e| format!("Failed to output PNG: {:#?}", e))?;
    let mut encoded_output_bytes = Vec::new();
    let mut c = file.as_slice();
    c.read_to_end(&mut encoded_output_bytes).unwrap();
    Ok(encoded_output_bytes)
}

#[cfg(test)]
pub mod resize_image {
    use super::*;

    #[test]
    fn test_output_png() {
        for dimensions in [(200, 100), (1, 1), (200, 1)].iter() {
            let img = DynamicImage::new_rgba8(dimensions.0, dimensions.1);
            output_to_png(&img).unwrap();
        }
    }
}
