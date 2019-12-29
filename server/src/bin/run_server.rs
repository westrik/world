#[macro_use]
extern crate log;

use std::env;
use async_std::task;

use dotenv::dotenv;

use timeline_server::db;
use timeline_server::routes;

/*
TODO(next):
 - add user model
 - add methods for password hashing + authenticating
 - hook up sign-in endpoint

TODO(later):
 - fix error handling once Tide supports that
 - use Tide middleware for DB pool and logging
*/

fn main() -> Result<(), std::io::Error> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = db::init_pool(&database_url).expect("Failed to create pool");

    let mut app = tide::with_state(pool);
    app.at("/").get(|_| async move { "Hello, world!" });
    task::block_on(async {
        app.listen("127.0.0.1:8080").await?;
        Ok(())
    })
}
