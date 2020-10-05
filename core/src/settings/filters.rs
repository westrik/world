use warp::Filter;

use crate::db::DbPool;
use crate::settings::sites::filters::routes as sites_routes;

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    sites_routes(db_pool.clone())
}
