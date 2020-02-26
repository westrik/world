use chrono::{DateTime, Utc};
use diesel::prelude::*;
use diesel::PgConnection;

use crate::auth::models::session::Session;
use crate::auth::models::user::User;
use crate::resource_identifier::{generate_resource_identifier, ResourceType};
use crate::schema::{sessions, sessions::dsl::sessions as all_sessions};
use crate::schema::{tasks, tasks::dsl::tasks as all_tasks};
use diesel::dsl::now;

/** Database model for Tasks


# Task sorting

By default, tasks are sorted by creation time in ascending order.
Tasks have optional `sibling_id` and `parent_id` fields.
When set, these influence the structure and ordering of the task tree.
The `sibling_id` field links a task to the task immediately above it.
The `parent_id` field links a task to the task immediately above and to the left of it.

**Constraint**: No two tasks can have the same `sibling_id` or `parent_id`.

#### Example

Consider four tasks: `A`, `B`, `C`, and `D`. The tasks were created one at a time, in alphabetical order.
The default sorting of tasks would be:

```txt
A
B
C
D
```

If we set `sibling_id` of `D` to the ID of `A`, then the sorting would become:

```txt
A
D
B
C
```

If we set `parent_id` of `C` to the ID of `B`, then the sorting would become:

```txt
A
D
B
  C
```






*/

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

#[derive(Debug, PartialEq)]
pub enum TaskError {
    TaskNotFound,
    InvalidToken,
    DatabaseError(diesel::result::Error),
}

#[derive(Insertable, Debug)]
#[table_name = "tasks"]
pub struct TaskCreateSpec {
    pub api_id: String,
    pub user_id: i32,
    pub description: String,
}
impl TaskCreateSpec {
    pub fn insert(&self, conn: &PgConnection) -> Result<Task, TaskError> {
        info!("creating task: {:?}", self);
        Ok(diesel::insert_into(tasks::table)
            .values(self)
            .get_result(conn)
            .map_err(TaskError::DatabaseError)?)
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
}
impl TaskUpdateSpec {
    pub fn update(
        &self,
        conn: &PgConnection,
        api_id: String,
        user_id: i32,
    ) -> Result<Task, TaskError> {
        info!("updating task {} with {:?}", api_id, self);
        Ok(diesel::update(
            all_tasks
                .filter(tasks::api_id.eq(&api_id))
                .filter(tasks::user_id.eq(user_id)),
        )
        .set(self)
        .get_result::<Task>(conn)
        .map_err(TaskError::DatabaseError)?)
    }
}

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
#[derive(Debug, Deserialize)]
pub struct ApiTaskCreateSpec {
    pub description: String,
}
#[derive(Debug, Deserialize)]
#[allow(non_snake_case)]
#[allow(clippy::option_option)]
pub struct ApiTaskUpdateSpec {
    pub description: Option<String>,
    pub parentApiId: Option<Option<String>>,
    pub siblingApiId: Option<Option<String>>,
    pub isCompleted: Option<bool>,
    pub isCollapsed: Option<bool>,
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
impl From<&Task> for ApiTask {
    fn from(task: &Task) -> Self {
        ApiTask {
            apiId: task.api_id.clone(),
            description: task.description.clone(),
            createdAt: task.created_at,
            updatedAt: task.updated_at,
            completedAt: task.completed_at,
            siblingApiId: None,
            parentApiId: None,
            isCollapsed: task.is_collapsed,
        }
    }
}

impl Task {
    pub fn find_all_for_user(conn: &PgConnection, token: String) -> Result<Vec<Task>, TaskError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| TaskError::TaskNotFound)?;
        let items: Vec<Task> = all_tasks
            .filter(tasks::user_id.eq(session.user_id))
            .filter(tasks::completed_at.is_null())
            .order(tasks::created_at.asc())
            .load(conn)
            .map_err(|_| TaskError::TaskNotFound)?;
        Ok(items)
    }

    pub fn create(
        conn: &PgConnection,
        token: String,
        description: String,
    ) -> Result<Task, TaskError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| TaskError::InvalidToken)?;
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
    ) -> Result<Task, TaskError> {
        // TODO: refactor this out
        let session: Session = all_sessions
            .filter(sessions::token.eq(token))
            .filter(sessions::expires_at.gt(now))
            .first(conn)
            .map_err(|_| TaskError::InvalidToken)?;

        TaskUpdateSpec {
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
        }
        .update(conn, api_id, session.user_id)
    }
}

/* ----- DB integration tests -----  */
