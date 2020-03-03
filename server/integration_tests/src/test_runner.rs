use crate::db::{begin_txn, create_test_db, destroy_test_db, rollback_txn, DbPool};
use std::io::{stdout, Write};

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn(DbPool)]) {
    println!("⚙️  setting up environment for integration tests...");
    let pool = create_test_db();

    print!("\n📋 running {} integration tests: ", tests.len());
    stdout().flush().unwrap();

    for test in tests {
        begin_txn(&pool);
        test(pool.clone());
        rollback_txn(&pool);
        print!(".");
        stdout().flush().unwrap();
    }
    print!("\n\n");
    destroy_test_db(&pool);
    print!("\n");
}
