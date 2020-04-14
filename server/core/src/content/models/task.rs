use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::errors::ApiError;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{tasks, tasks::dsl::tasks as all_tasks};

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
    pub block_id: i32,
}
pub struct LoadedTask {
    pub task: Task,
    pub parent_api_id: Option<String>,
    pub sibling_api_id: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct TaskCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub block_id: i32,
    pub description: String,
}
impl TaskCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Task, ApiError> {
        info!("creating task: {:?}", self);
        Ok(diesel::insert_into(tasks::table)
            .values(self)
            .get_result(conn)
            .map_err(ApiError::DatabaseError)?)
    }
}

#[allow(clippy::option_option)]
#[derive(AsChangeset, Debug)]
#[table_name = "tasks"]
pub struct TaskUpdateSpec {
    pub updated_at: DateTime<Utc>,
    pub completed_at: Option<Option<DateTime<Utc>>>,
    pub description: Option<String>,
    pub is_collapsed: Option<bool>,
    pub parent_id: Option<Option<i32>>,
    pub sibling_id: Option<Option<i32>>,
    pub block_id: Option<i32>,
}
impl TaskUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<Task, ApiError> {
        info!("updating task {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_tasks
                .filter(tasks::api_id.eq(&api_id))
                .filter(tasks::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<Task>(conn)
        .map_err(ApiError::DatabaseError)?)
    }
}

#[derive(Serialize)]
pub struct ApiTask {
    #[serde(rename = "apiId")]
    pub api_id: String,
    pub description: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    #[serde(rename = "completedAt")]
    pub completed_at: Option<DateTime<Utc>>,
    #[serde(rename = "siblingApiId")]
    pub sibling_api_id: Option<String>,
    #[serde(rename = "parentApiId")]
    pub parent_api_id: Option<String>,
    #[serde(rename = "isCollapsed")]
    pub is_collapsed: bool,
}
#[derive(Debug, Deserialize)]
pub struct ApiTaskCreateSpec {
    pub description: String,
}
#[derive(Debug, Deserialize)]
#[allow(clippy::option_option)]
pub struct ApiTaskUpdateSpec {
    pub description: Option<String>,
    #[serde(rename = "parentApiId")]
    pub parent_api_id: Option<Option<String>>,
    #[serde(rename = "siblingApiId")]
    pub sibling_api_id: Option<Option<String>>,
    #[serde(rename = "isCompleted")]
    pub is_completed: Option<bool>,
    #[serde(rename = "isCollapsed")]
    pub is_collapsed: Option<bool>,
}

impl From<LoadedTask> for ApiTask {
    fn from(lt: LoadedTask) -> Self {
        ApiTask {
            api_id: lt.task.api_id,
            description: lt.task.description,
            created_at: lt.task.created_at,
            updated_at: lt.task.updated_at,
            completed_at: lt.task.completed_at,
            sibling_api_id: lt.sibling_api_id,
            parent_api_id: lt.parent_api_id,
            is_collapsed: lt.task.is_collapsed,
        }
    }
}
impl From<&Task> for ApiTask {
    fn from(task: &Task) -> Self {
        ApiTask {
            api_id: task.api_id.clone(),
            description: task.description.clone(),
            created_at: task.created_at,
            updated_at: task.updated_at,
            completed_at: task.completed_at,
            sibling_api_id: None,
            parent_api_id: None,
            is_collapsed: task.is_collapsed,
        }
    }
}

impl Task {
    pub fn find_all_for_user(conn: &PgConnection, session: Session) -> Result<Vec<Task>, ApiError> {
        let items: Vec<Task> = all_tasks
            .filter(tasks::user_id.eq(session.user_id))
            .load(conn)
            .map_err(ApiError::DatabaseError)?;
        Ok(items)
    }

    pub fn create(
        conn: &PgConnection,
        session: Session,
        description: String,
    ) -> Result<Task, ApiError> {
        let new_task = TaskCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Task),
            user_id: session.user_id,
            block_id: 1, // TODO: block ID
            description,
        };
        new_task.insert(conn)
    }

    pub fn update(
        conn: &PgConnection,
        session: Session,
        api_id: String,
        spec: ApiTaskUpdateSpec,
    ) -> Result<Task, ApiError> {
        TaskUpdateSpec {
            block_id: None,
            updated_at: Utc::now(),
            completed_at: match spec.is_completed {
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
            is_collapsed: spec.is_collapsed,
            parent_id: None,
            sibling_id: None,
        }
        .update(conn, api_id, session.user_id)
    }
}
