use crate::db::{get_conn, DbPool};
use crate::fixtures::{get_test_session, get_test_user};
use westrikworld_core::resource_identifier::{generate_resource_identifier, ResourceType};
use westrikworld_core::tasks::models::task::{Task, TaskCreateSpec};

// TODO:
//  - test update logic
//  - test parent-child logic

#[test_case]
fn test_task_create_and_get(pool: &DbPool) {
    let conn = get_conn(pool).unwrap();
    let user = get_test_user(&conn);

    for id in 0..10 {
        let new_task = TaskCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Task),
            user_id: user.id,
            description: format!("HELLO WORLD {}", id).to_string(),
        };
        new_task.insert(&conn).unwrap();
    }

    let session = get_test_session(&conn, user.id);
    let tasks = Task::find_all_for_user(&conn, session.token).unwrap();

    assert_eq!(tasks.len(), 10);
}
