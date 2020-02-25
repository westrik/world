use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use crate::schema::{tasks, tasks::dsl::tasks as all_tasks};
use diesel::dsl::now;

/* ----- Model definitions -----  */

#[derive(Associations, Identifiable, Queryable, Serialize, Deserialize, Debug)]
#[belongs_to(User)]
pub struct Task {
    pub id: i32,
    pub api_id: String,
    pub user_id: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
    pub description: String,
    pub sibling_id: Option<i32>,
    pub parent_id: Option<i32>,
    pub is_collapsed: bool,
}
pub struct LoadedTask {
    pub task: Task,
    pub parent_api_id: Option<String>,
    pub sibling_api_id: Option<String>,
}

/* ----- Query helper structs  -----  */

#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
#[derive(Debug)]
pub enum TaskQueryError {
    TaskNotFound,
    InvalidToken,
    DatabaseError(diesel::result::Error),
}

/* ----- Create and update specs  -----  */

#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct TaskCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub description: String,
}
impl TaskCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Task, TaskQueryError> {
        info!("{:?}", self);
        Ok(diesel::insert_into(tasks::table)
            .values(self)
            .get_result(conn)
            .map_err(TaskQueryError::DatabaseError)?)
    }
}
#[derive(Debug, Deserialize)]
pub struct ApiTaskCreateSpec {
    pub description: String,
}
#[derive(AsChangeset, Debug)]
#[table_name = "tasks"]
pub struct TaskUpdateSpec {
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<Option<DateTime<Utc>>>,
    pub description: Option<String>,
    pub is_collapsed: Option<bool>,
    pub parent_id: Option<Option<i32>>,
    pub sibling_id: Option<Option<i32>>,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
pub struct ApiTaskUpdateSpec {
    pub description: Option<String>,
    pub parentApiId: Option<Option<String>>,
    pub siblingApiId: Option<Option<String>>,
    pub isCompleted: Option<bool>,
    pub isCollapsed: Option<bool>,
}

/* ----- API interfaces -----  */

#[derive(Serialize)]
#[allow(non_snake_case)]
pub struct ApiTask {
    pub apiId: String,
    pub description: String,
    pub createdAt: DateTime<Utc>,
    pub updatedAt: DateTime<Utc>,
    pub completedAt: Option<DateTime<Utc>>,
    pub siblingApiId: Option<String>,
    pub parentApiId: Option<String>,
    pub isCollapsed: bool,
}
impl From<LoadedTask> for ApiTask {
    fn from(lt: LoadedTask) -> Self {
        ApiTask {
            apiId: lt.task.api_id,
            description: lt.task.description,
            createdAt: lt.task.created_at,
            updatedAt: lt.task.updated_at,
            completedAt: lt.task.completed_at,
            siblingApiId: lt.sibling_api_id,
            parentApiId: lt.parent_api_id,
            isCollapsed: lt.task.is_collapsed,
        }
    }
}

/* ----- DB business logic -----  */

impl Task {
    pub fn find_all_for_user(
        conn: &PgConnection,
        token: String,
    ) -> Result<Vec<Task>, TaskQueryError> {
        // TODO: refactor this out
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
        description: String,
    ) -> Result<Task, TaskQueryError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| TaskQueryError::InvalidToken)?;
        let new_task = TaskCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Task),
            user_id: session.user_id,
            description,
        };
        new_task.insert(conn)
    }

    pub fn update(
        conn: &PgConnection,
        token: String,
        api_id: String,
        spec: ApiTaskUpdateSpec,
    ) -> Result<Task, TaskQueryError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| TaskQueryError::InvalidToken)?;

        let update_spec = TaskUpdateSpec {
            updated_at: Utc::now(),
            completed_at: match spec.isCompleted {
                Some(is_completed) => {
                    if is_completed {
                        Some(Some(Utc::now()))
                    } else {
                        Some(None)
                    }
                }
                None => None,
            },
            description: spec.description,
            is_collapsed: spec.isCollapsed,
            parent_id: None,
            sibling_id: None,
        };
        Ok(diesel::update(
            all_tasks
                .filter(tasks::api_id.eq(&api_id))
                .filter(tasks::user_id.eq(session.user_id)),
        )
        .set(update_spec)
        .get_result::<Task>(conn)
        .map_err(TaskQueryError::DatabaseError)?)
    }
}

/* ----- DB integration tests -----  */

#[cfg(test)]
pub mod test_task_model {
    use crate::resource_identifier::*;
    use crate::tasks::models::task::TaskCreateSpec;
    use crate::test_utils::db::{connect_to_test_db, get_conn, rollback};
    use crate::test_utils::fixtures::create_test_user;

    #[test]
    fn test_task_create() {
        let pool = connect_to_test_db();

        let conn = get_conn(&pool).unwrap();

        let test_user = create_test_user(&conn);

        let new_task = TaskCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Task),
            user_id: test_user.id,
            description: "HELLO WORLD".to_string(),
        };
        println!("🗒 Inserting test task");
        new_task.insert(&conn).unwrap();

        rollback(&pool);
    }

    // TODO: test update logic

    // TODO: test parent-child logic
}
