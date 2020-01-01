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

#[derive(Insertable, Debug)]
#[table_name = "items"]
pub struct NewItem {
    pub user_id: i32,
    pub content: String,
}

impl NewItem {
    pub fn insert(&self, conn: &PgConnection) -> Result<Item, ItemQueryError> {
        info!("{:?}", self);
        Ok(diesel::insert_into(items::table)
            .values(self)
            .get_result(conn)
            .map_err(|err| ItemQueryError::DatabaseError(err))?)
    }
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
        let items: Vec<Item> = all_items
            .filter(items::user_id.eq(session.user_id))
            .load(conn)
            .map_err(|_| ItemQueryError::ItemNotFound)?;
        Ok(items)
    }

    pub fn create(
        conn: &PgConnection,
        token: String,
        content: String,
    ) -> Result<Item, ItemQueryError> {
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| ItemQueryError::InvalidToken)?;
        let new_user = NewItem {
            user_id: session.user_id,
            content: content,
        };
        new_user.insert(conn)
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
