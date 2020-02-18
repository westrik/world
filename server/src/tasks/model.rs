use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use crate::schema::{tasks, tasks::dsl::tasks as all_tasks};
use diesel::dsl::now;

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Task {
    pub id: i32,
    pub user_id: i32,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub api_id: Option<String>,
    pub next_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub is_collapsed: bool,
}

#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct NewTask {
    pub api_id: String,
    pub user_id: i32,
    pub content: String,
}

impl NewTask {
    pub fn insert(&self, conn: &PgConnection) -> Result<Task, TaskQueryError> {
        info!("{:?}", self);
        Ok(diesel::insert_into(tasks::table)
            .values(self)
            .get_result(conn)
            .map_err(TaskQueryError::DatabaseError)?)
    }
}

#[derive(Debug)]
pub enum TaskQueryError {
    TaskNotFound,
    InvalidToken,
    DatabaseError(diesel::result::Error),
}

impl Task {
    pub fn find_all_for_user(
        conn: &PgConnection,
        token: String,
    ) -> Result<Vec<Task>, TaskQueryError> {
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| TaskQueryError::TaskNotFound)?;
        let items: Vec<Task> = all_tasks
            .filter(tasks::user_id.eq(session.user_id))
            .load(conn)
            .map_err(|_| TaskQueryError::TaskNotFound)?;
        Ok(items)
    }

    pub fn create(
        conn: &PgConnection,
        token: String,
        content: String,
    ) -> Result<Task, TaskQueryError> {
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| TaskQueryError::InvalidToken)?;
        let new_task = NewTask {
            api_id: generate_resource_identifier(ResourceType::Task),
            user_id: session.user_id,
            content,
        };
        new_task.insert(conn)
    }
}

#[derive(Serialize)]
pub struct UiItem {
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Task> for UiItem {
    fn from(item: Task) -> Self {
        UiItem {
            content: item.content,
            created_at: item.created_at,
            updated_at: item.updated_at,
        }
    }
}

#[cfg(test)]
pub mod test_task_model {
    //    use super::*;

    // TODO: spin up (+ destroy) test database

    #[test]
    fn test_task_create() {}
}
