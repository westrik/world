use crate::db::{get_conn, PgPool};
use crate::fixtures::create_test_user;
use westrikworld_core::resource_identifier::{generate_resource_identifier, ResourceType};
use westrikworld_core::tasks::models::task::TaskCreateSpec;

#[test_case]
fn test_task_create(pool: &PgPool) {
    let conn = get_conn(pool).unwrap();

    let test_user = create_test_user(&conn);

    let new_task = TaskCreateSpec {
        api_id: generate_resource_identifier(ResourceType::Task),
        user_id: test_user.id,
        description: "HELLO WORLD".to_string(),
    };
    println!("🗒 Inserting test task");
    new_task.insert(&conn).unwrap();
}

// TODO: test update logic

// TODO: test parent-child logic