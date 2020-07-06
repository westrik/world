use crate::db::{get_conn, DbConnection, DbPool};
use crate::fixtures::auth::{get_test_session, get_test_user};
// use crate::fixtures::blocks::create_test_block;
use westrikworld_core::content::models::task::{Task, TaskCreateSpec};
use westrikworld_core::resource_identifier::{generate_resource_identifier, ResourceType};

// TODO:
//  - test update logic
//  - test parent-child logic

fn create_n_tasks(conn: &DbConnection, n: i32, user_id: i32, block_id: i32) {
    for id in 0..n {
        let new_task = TaskCreateSpec {
            api_id: generate_resource_identifier(ResourceType::Task),
            user_id,
            block_id,
            description: format!("HELLO WORLD #{:04}", id).to_string(),
        };
        new_task.insert(conn).unwrap();
    }
}
//
// #[test_case]
// fn test_task_create_and_get(pool: &DbPool) {
//     let conn = get_conn(pool).unwrap();
//     let user = get_test_user(&conn);
//     let block = create_test_block(&conn, user.id);
//     create_n_tasks(&conn, 10, user.id, block.id);
//
//     let tasks = Task::find_all_for_user(&conn, get_test_session(&conn, user.id)).unwrap();
//
//     assert_eq!(tasks.len(), 10);
// }