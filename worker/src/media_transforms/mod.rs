pub mod convert;
pub mod resize;

#[cfg(test)]
pub mod convert_and_resize_images {
    use image::DynamicImage;
    use std::collections::HashMap;

    use super::*;
    use crate::media_transforms::convert::output_to_png;

    use std::fs::File;
    use std::io::Read;

    #[test]
    fn test_resize() {
        let img = image::open("./src/media_transforms/test_data/rgba.png").unwrap();

        let widths = vec![400, 1000, 2750];

        let size_to_resized_img: HashMap<u32, DynamicImage> =
            resize::constrain_image_dimensions(img, widths);

        for img in size_to_resized_img.iter() {
            let mut expected_img_file = File::open(format!(
                "./src/media_transforms/test_data/rgba-{}.png",
                img.0
            ))
            .unwrap();
            let mut expected_img = Vec::new();
            expected_img_file.read_to_end(&mut expected_img).unwrap();

            let resized_img = output_to_png(img.1).unwrap();
            assert_eq!(resized_img, expected_img);
        }
    }
}
