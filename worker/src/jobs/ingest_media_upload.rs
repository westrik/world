use async_trait::async_trait;
use image::ImageFormat;
use std::io::Read;

use world_core::external_services::aws::s3::{get_object::get_object, put_object::put_object};
use world_core::jobs::errors::JobError;
use world_core::library::models::file::FileType;
use world_core::utils::config::CONTENT_BUCKET_NAME;

use crate::jobs::Runnable;
use crate::media_transforms::resize::constrain_image_dimensions;

#[derive(Deserialize)]
pub struct IngestMediaUploadJob {
    pub file_name: String,
    pub library_version_api_id: String,
}

fn file_name_for_resized_version(file_name: &str, width: u32) -> String {
    let path_segments: Vec<&str> = file_name.split('.').into_iter().collect();
    let (file_name_stem, file_extension) = path_segments.split_at(path_segments.len() - 1);
    format!(
        "{}-{}.{}",
        file_name_stem.concat(),
        width,
        file_extension.concat()
    )
}

#[async_trait]
impl Runnable for IngestMediaUploadJob {
    async fn run(&self) -> Result<String, JobError> {
        info!(
            "Running media upload job for {}",
            self.library_version_api_id
        );
        let object = get_object(CONTENT_BUCKET_NAME.to_string(), &self.file_name)
            .await
            .map_err(|e| {
                JobError::InternalError(format!(
                    "Failed to get uploaded item from S3 [error={:#?}",
                    e
                ))
            })?;
        let object_type = FileType::from(object.content_type.unwrap());
        let image_format = match object_type {
            FileType::GIF => Ok(ImageFormat::Gif),
            FileType::JPEG => Ok(ImageFormat::Jpeg),
            FileType::PNG => Ok(ImageFormat::Png),
            FileType::TIFF => Ok(ImageFormat::Tiff),
            // FileType::WEBP => {
            //     // This might only decode luma channel right now
            //     Ok(ImageFormat::WebP)
            // }
            // TODO: for FileType::{EPUB, MPEG, PDF, SVG, WEBM}, enqueue job to generate preview
            _ => Err(JobError::InvalidJob(
                "Uploaded file was not an image".to_string(),
            )),
        }?;
        let mut object_bytes = Vec::new();
        object
            .body
            .unwrap()
            .into_blocking_read()
            .read_to_end(&mut object_bytes)
            .unwrap();
        let image =
            image::load_from_memory_with_format(&object_bytes, image_format).map_err(|e| {
                JobError::InternalError(format!("Failed to load image [error={:#?}", e))
            })?;
        // TODO: insert corresponding DB rows
        let resized_image_map = constrain_image_dimensions(image, vec![1200, 720, 360]);
        for (width, resized_image) in resized_image_map {
            let new_file_name = file_name_for_resized_version(&self.file_name, width);
            let put_resp = put_object(
                CONTENT_BUCKET_NAME.to_string(),
                &new_file_name,
                resized_image.to_bytes(),
            )
            .await;
            info!("{:#?}", put_resp);
        }
        Ok("DONE".to_string())
    }
}

#[cfg(test)]
pub mod ingest_media_upload_job {
    use super::*;

    #[test]
    fn test_resized_file_name() {
        assert_eq!(
            "user_asdfasdf/asdfasdf-1000.png".to_string(),
            file_name_for_resized_version("user_asdfasdf/asdfasdf.png", 1000)
        );
    }
}
