use crate::db::{connect_to_test_db, destroy_test_db, rollback_txn, start_txn, PgPool};

#[cfg(test)]
pub fn runner(ts: &[&dyn Fn(&PgPool)]) {
    println!("running {} tests with DB connection:\n", ts.len());
    let pool = connect_to_test_db();
    for t in ts {
        start_txn(&pool);
        t(&pool);
        rollback_txn(&pool);
        print!("\n");
    }
    destroy_test_db(&pool);
}

pub fn describe_test(desc: &str) {
    println!("{}", desc);
}
