use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::models::session::Session;
use crate::models::user::User;
use crate::schema::{items, items::dsl::items as all_items};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use diesel::dsl::now;

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Item {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub enum ItemQueryError {
    ItemNotFound,
    InvalidToken,
    DatabaseError(diesel::result::Error),
}

impl Item {
    pub fn find_all_for_user(
        conn: &PgConnection,
        token: String,
    ) -> Result<Vec<Item>, ItemQueryError> {
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| ItemQueryError::ItemNotFound)?;

        info!("{:?}", session);

        Ok(Vec::new())
    }
}

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
