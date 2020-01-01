use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::user::User;
use crate::schema::{items, items::dsl::items as all_items};

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize)]
#[belongs_to(User)]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Item {}

#[derive(Serialize)]
pub struct UiItem {
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Item> for UiItem {
    fn from(item: Item) -> Self {
        UiItem {
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}
