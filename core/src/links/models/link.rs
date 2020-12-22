use chrono::{DateTime, Utc};
use diesel::insert_into;
use diesel::prelude::*;

use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::notes::models::note::Note;
use crate::schema::{links, links::dsl::links as all_links};
// use crate::utils::list_options::ListOptions;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(Note)]
#[belongs_to(User)]
#[table_name = "links"]
pub struct LinkSummary {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(skip)]
    pub note_id: i32,
    #[serde(skip)]
    pub note_version_id: i32,
    #[serde(skip)]
    pub media_item_id: Option<i32>,
    #[serde(skip)]
    pub target_note_id: Option<i32>,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Link {
    #[serde(skip)]
    pub id: i32,
    #[serde(rename = "id")]
    pub api_id: String,
    #[serde(skip)]
    pub user_id: i32,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(skip)]
    pub note_id: i32,
    #[serde(rename = "noteId")]
    pub note_api_id: String,
    #[serde(skip)]
    pub note_version_id: i32,
    #[serde(rename = "noteVersionId")]
    pub note_version_api_id: String,
    #[serde(rename = "mediaItemId")]
    pub media_item_api_id: Option<String>,
    #[serde(skip)]
    pub media_item_id: Option<i32>,
    #[serde(rename = "targetNoteId")]
    pub target_note_api_id: Option<String>,
    #[serde(skip)]
    pub target_note_id: Option<i32>,
    #[serde(rename = "externalUrl")]
    pub external_url: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "links"]
pub struct LinkCreateSpec {
    pub user_id: i32,
    pub note_id: i32,
    pub note_version_id: i32,
    pub media_item_id: Option<i32>,
    pub target_note_id: Option<i32>,
    pub external_url: Option<String>,
}

// impl LinkSummary {
//     pub fn find(
//         conn: &PgConnection,
//         user_id: i32,
//         note_api_id: String,
//     ) -> Result<LinkSummary, ApiError> {
//         Ok(all_media_items
//             .filter(media_items::api_id.eq(&api_id))
//             .filter(media_items::user_id.eq(user_id))
//             .first::<MediaItemSummary>(conn)
//             .map_err(ApiError::DatabaseError)?)
//     }
// }

impl Link {
    pub fn bulk_create(
        conn: &PgConnection,
        specs: Vec<LinkCreateSpec>,
    ) -> Result<Vec<LinkSummary>, ApiError> {
        Ok(insert_into(all_links)
            .values(specs)
            .get_results::<LinkSummary>(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}
