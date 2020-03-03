use crate::db::{create_test_db, destroy_test_db, rollback_txn, start_txn, DbPool};

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn(&DbPool)]) {
    println!("⚙️  setting up environment for integration tests...");
    let pool = create_test_db();

    println!("\n📋 running {} integration tests\n", tests.len());
    for test in tests {
        let pool = pool.clone();
        start_txn(&pool);
        test(&pool);
        rollback_txn(&pool);
        print!(".");
    }
    print!("\n\n");
    destroy_test_db(&pool);
    print!("\n");
}
