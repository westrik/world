use async_trait::async_trait;
use image::ImageFormat;
use std::io::Read;

use crate::jobs::Runnable;
use world_core::external_services::aws::s3::{get_object::get_object, put_object::put_object};
use world_core::jobs::errors::JobError;
use world_core::library::models::file::FileType;
use world_core::utils::config::CONTENT_BUCKET_NAME;

#[derive(Deserialize)]
pub struct IngestMediaUploadJob {
    pub file_name: String,
    pub library_version_api_id: String,
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
        let _image =
            image::load_from_memory_with_format(&object_bytes, image_format).map_err(|e| {
                JobError::InternalError(format!("Failed to load image [error={:#?}", e))
            })?;
        // let resized_image = constrain_image_dimensions(image);

        /* <parallelize> */
        // TODO: resize and convert
        let new_file_name = "/aslkdjfkalsdf";
        // TODO: upload results to S3 and insert corresponding DB rows
        let _put_resp = put_object(CONTENT_BUCKET_NAME.to_string(), new_file_name).await;
        /* <parallelize> */
        Ok("DONE".to_string())
    }
}
