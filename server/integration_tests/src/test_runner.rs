use crate::db::{connect_to_test_db, destroy_test_db, rollback, PgPool};

#[cfg(test)]
pub fn runner(ts: &[&dyn Fn(&PgPool)]) {
    println!("Running {} tests: ", ts.len());
    let pool = connect_to_test_db();
    for t in ts {
        println!("Running test");
        t(&pool);
        rollback(&pool);
    }
    destroy_test_db(&pool);
}
