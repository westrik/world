use crate::db::{get_conn, DbConnection, DbPool};
use crate::fixtures::{get_test_session, get_test_user};
use westrikworld_core::resource_identifier::{generate_resource_identifier, ResourceType};
use westrikworld_core::tasks::models::task::{Task, TaskCreateSpec, TaskError};

// TODO:
//  - test update logic
//  - test parent-child logic

fn create_n_tasks(conn: &DbConnection, n: i32, user_id: i32) {
    for id in 0..n {
        let new_task = TaskCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Task),
            user_id,
            description: format!("HELLO WORLD #{:04}", id).to_string(),
        };
        new_task.insert(conn).unwrap();
    }
}

#[test_case]
fn test_task_create_and_get(pool: DbPool) {
    let conn = get_conn(&pool).unwrap();
    let user = get_test_user(&conn);
    create_n_tasks(&conn, 10, user.id);

    let session = get_test_session(&conn, user.id);
    let tasks = Task::find_all_for_user(&conn, session.token).unwrap();

    assert_eq!(tasks.len(), 10);
}

#[test_case]
fn test_get_tasks_invalid_token(pool: DbPool) {
    let conn = get_conn(&pool).unwrap();
    let user = get_test_user(&conn);
    create_n_tasks(&conn, 10, user.id);

    let result = Task::find_all_for_user(&conn, "INVALID_TOKEN".to_string());
    // TODO: should be InvalidToken
    assert_eq!(result.unwrap_err(), TaskError::TaskNotFound);
}
