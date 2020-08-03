use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::jobs::job_type::JobType;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{jobs, jobs::dsl::jobs as all_jobs};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Job {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub job_type: String,
    pub payload: Option<serde_json::Value>,
    #[serde(skip)]
    pub user_id: Option<i32>,
}

#[derive(Insertable, Debug)]
#[table_name = "jobs"]
pub struct JobCreateSpec {
    pub api_id: String,
    pub user_id: Option<i32>,
    pub job_type: String,
    pub payload: Option<serde_json::Value>,
}

impl JobCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Job, ApiError> {
        info!("creating job: {:#?}", self);
        Ok(diesel::insert_into(jobs::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

impl Job {
    pub fn find(conn: &PgConnection, api_id: String) -> Result<Job, ApiError> {
        // TODO: filter by user id
        let job = all_jobs
            .filter(jobs::api_id.eq(&api_id))
            .first(conn)
            .map_err(|e| match e {
                diesel::result::Error::NotFound => ApiError::NotFound(api_id),
                _ => ApiError::DatabaseError(e),
            })?;
        Ok(job)
    }

    pub fn create(
        conn: &PgConnection,
        user_id: Option<i32>,
        job_type: JobType,
        payload: Option<serde_json::Value>,
    ) -> Result<Job, ApiError> {
        JobCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Job),
            user_id,
            job_type: job_type.to_string(),
            payload,
        }
        .insert(conn)
    }
}
