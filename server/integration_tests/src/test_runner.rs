use crate::db::{connect_to_test_db, destroy_test_db, get_conn, rollback_txn, start_txn, DbPool};
use crate::fixtures::create_test_user;

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn(&DbPool)]) {
    println!("running {} tests with DB connection:\n", tests.len());

    let pool = connect_to_test_db();

    let conn = get_conn(&pool).unwrap();
    create_test_user(&conn);

    for test in tests {
        let pool = pool.clone();
        start_txn(&pool);
        test(&pool);
        rollback_txn(&pool);
        print!("\n");
    }
    destroy_test_db(&pool);
}
