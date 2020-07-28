use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::schema::{library_items, library_items::dsl::library_items as all_library_items};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct LibraryItem {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub name: String,
    #[serde(rename = "preSignedUploadUrl")]
    pub presigned_upload_url: Option<String>,
    #[serde(rename = "uploadedFileName")]
    pub uploaded_file_name: Option<String>,
    #[serde(rename = "uploadedFileSizeBytes")]
    pub uploaded_file_size_bytes: Option<i64>,
}

#[derive(Insertable, Debug)]
#[table_name = "library_items"]
pub struct LibraryItemCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub name: String,
    pub presigned_upload_url: Option<String>,
    pub uploaded_file_name: Option<String>,
    pub uploaded_file_size_bytes: Option<i64>,
}

#[derive(AsChangeset, Debug)]
#[table_name = "library_items"]
pub struct LibraryItemUpdateSpec {
    pub updated_at: DateTime<Utc>, // TODO: use trigger to set updated_at automatically
    pub name: Option<String>,
}
impl LibraryItemUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<LibraryItem, ApiError> {
        info!("updating library_item {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_library_items
                .filter(library_items::api_id.eq(&api_id))
                .filter(library_items::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<LibraryItem>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

impl LibraryItem {
    pub fn find_all(conn: &PgConnection, session: Session) -> Result<Vec<LibraryItem>, ApiError> {
        Ok(all_library_items
            .filter(library_items::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?)
    }

    pub fn find(
        conn: &PgConnection,
        session: Session,
        api_id: String,
    ) -> Result<LibraryItem, ApiError> {
        Ok(all_library_items
            .filter(library_items::user_id.eq(session.user_id))
            .filter(library_items::api_id.eq(api_id))
            .first::<LibraryItem>(conn)
            .map_err(ApiError::DatabaseError)?)
    }

    pub fn bulk_create(
        conn: &PgConnection,
        specs: Vec<LibraryItemCreateSpec>,
    ) -> Result<Vec<LibraryItem>, ApiError> {
        Ok(insert_into(all_library_items)
            .values(specs)
            .get_results(conn)
            .map_err(ApiError::DatabaseError)?)
    }

    pub fn update(
        session: Session,
        conn: &PgConnection,
        api_id: String,
        name: Option<String>,
    ) -> Result<LibraryItem, ApiError> {
        LibraryItemUpdateSpec {
            updated_at: Utc::now(),
            name,
        }
        .update(conn, api_id, session.user_id)
    }
}
