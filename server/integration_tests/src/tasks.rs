use crate::db::{get_conn, DbPool};
use crate::fixtures::get_test_user;
use westrikworld_core::resource_identifier::{generate_resource_identifier, ResourceType};
use westrikworld_core::tasks::models::task::TaskCreateSpec;

// TODO:
//  - test update logic
//  - test parent-child logic

#[test_case]
fn test_task_create(pool: &DbPool) {
    let conn = get_conn(pool).unwrap();
    let test_user = get_test_user(&conn);

    let new_task = TaskCreateSpec {
        api_id: generate_resource_identifier(ResourceType::Task),
        user_id: test_user.id,
        description: "HELLO WORLD".to_string(),
    };
    println!("inserting test task");
    new_task.insert(&conn).unwrap();
}
