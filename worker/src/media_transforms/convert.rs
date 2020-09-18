use image::png::PngEncoder;
use image::{ColorType, DynamicImage, GenericImageView, ImageBuffer, Pixel};
use std::io::{Read, Write};

pub fn output_to_png(
    image_buffer: DynamicImage,
    dimensions: (u32, u32),
    color_type: ColorType,
) -> Result<Vec<u8>, &'static str> {
    let mut file = Vec::new();

    // TODO: test new_with_quality w/ various CompressionType and FilterType
    let encoder = PngEncoder::new(file.by_ref());

    // TODO: use correct ColorType
    encoder
        .encode(
            &image_buffer.to_bytes(),
            dimensions.0,
            dimensions.1,
            color_type,
        )
        .map_err(|_| "Failed to encode PNG")?;

    let mut encoded_output_bytes = Vec::new();
    let mut c = file.as_slice();
    c.read_to_end(&mut encoded_output_bytes).unwrap();
    // cursor.read_to_end(&mut encoded_output_bytes).map_err(|_| "Couldn't read encoded PNG from memory")?;
    Ok(encoded_output_bytes)
}

#[allow(dead_code)]
pub fn content_based_crop<I: GenericImageView>(
    _image: I,
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
    I::Pixel: 'static,
    <I::Pixel as Pixel>::Subpixel: 'static,
{
    unimplemented!()
}

#[cfg(test)]
pub mod resize_image {
    use super::*;

    #[test]
    fn test_output_png() {
        let dimensions = (200, 100);
        let img = DynamicImage::new_rgba8(dimensions.0, dimensions.1);
        let resized_img = output_to_png(img, dimensions, ColorType::Rgba8).unwrap();

        assert_ne!(resized_img, vec![] as Vec<u8>);
    }
}
