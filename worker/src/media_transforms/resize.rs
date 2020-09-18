// use image::imageops::{resize, FilterType};
use image::DynamicImage;

pub fn constrain_image_dimensions(_image: DynamicImage) -> DynamicImage {
    // FilterType::CatmullRom and FilterType::Lanczos3 seem to give similar results, but
    // FilterType::CatmullRom is (anecdotally) about 3x faster.

    // TODO: convert to rgba8 first?

    // match image {
    //     DynamicImage::ImageBgr8(_) => {}
    //     DynamicImage::ImageBgra8(_) => {}
    //     DynamicImage::ImageLuma16(_) => {}
    //     DynamicImage::ImageLuma8(_) => {}
    //     DynamicImage::ImageLumaA16(_) => {}
    //     DynamicImage::ImageLumaA8(_) => {}
    //     DynamicImage::ImageRgb16(_) => {}
    //     DynamicImage::ImageRgb8(img) => DynamicImage::ImageRgb8(resized_buffer),
    //     DynamicImage::ImageRgba16(_) => {}
    //     DynamicImage::ImageRgba8(_) => {}
    // }
    //
    // convert from rgba8 back to actual type

    // let resized_buffer = resize(&image, width, height, FilterType::CatmullRom);

    unimplemented!()
}

// TODO: pub fn content_based_crop()

#[cfg(test)]
pub mod resize_image {
    // use super::*;
    // use image::{ImageBuffer, RgbImage};

    // TODO: test images w/ various properties
    // - animated gif
    // - non-animated gif
    // - png
    // - jpeg
    // - tiff
    // - webp
    // - 10-bit color (where applicable)

    #[test]
    fn test_constrain_image_dimensions() {
        // let img: RgbImage = ImageBuffer::new(2000, 1000);
        // let resized_img = constrain_image_dimensions(img);
        let (width, height) = (2000, 1000);
        assert_eq!(width, 2000);
        assert_eq!(height, 1000);
    }
}
