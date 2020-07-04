use crate::db::*;
use std::io::{stdout, Write};

#[cfg(test)]
pub fn runner(tests: &[&dyn Fn(&DbPool)]) {
    println!("⚙️  setting up environment for integration tests...");
    let pool = create_test_db();

    print!("\n📋 running {} integration tests: ", tests.len());
    stdout().flush().unwrap();

    for test in tests {
        let pool = pool.clone();
        begin_txn(&get_conn(&pool).unwrap()).unwrap();
        test(&pool);
        rollback_txn(&get_conn(&pool).unwrap()).unwrap();
        print!(".");
        stdout().flush().unwrap();
    }
    print!("\n\n");
    destroy_test_db(&pool);
    print!("\n");
}
