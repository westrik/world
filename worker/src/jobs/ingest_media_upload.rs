use async_trait::async_trait;
use world_core::jobs::errors::JobError;

use crate::jobs::Runnable;
use world_core::external_services::aws::s3::put_object::put_object;
use world_core::utils::config::CONTENT_BUCKET_NAME;

#[derive(Deserialize)]
pub struct IngestMediaUploadJob {
    pub file_name: String,
    pub library_version_api_id: String,
}

#[async_trait]
impl Runnable for IngestMediaUploadJob {
    async fn run(&self) -> Result<String, JobError> {
        info!("Running media upload job");
        // TODO: fetch original file from S3
        // let object = get_object(CONTENT_BUCKET_NAME.to_string(), &self.file_name).await?;
        // object.body?.
        /* <parallelize> */
        // TODO: resize and convert
        let new_file_name = "/aslkdjfkalsdf";
        // TODO: upload results to S3 and insert corresponding DB rows
        let _put_resp = put_object(CONTENT_BUCKET_NAME.to_string(), new_file_name).await;
        /* <parallelize> */
        Ok("DONE".to_string())
    }
}
