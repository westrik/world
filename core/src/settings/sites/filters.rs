use crate::db::DbPool;
use crate::routes::{json_body, with_db, with_session};
use crate::settings::sites::handlers;
use crate::settings::sites::models::site::{ApiSiteCreateSpec, ApiSiteUpdateSpec};
use crate::settings::sites::models::site_page::ApiSitePageCreateSpec;
use crate::utils::list_options::ListOptions;
use warp::Filter;

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    sites_list(db_pool.clone())
        .or(site_create(db_pool.clone()))
        .or(site_update(db_pool.clone()))
        .or(site_delete(db_pool.clone()))
        .or(site_pages_list(db_pool))
}

/// GET /site?offset=3&limit=5
pub fn sites_list(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("site")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::list_sites)
}

/// POST /site with JSON body
pub fn site_create(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("site")
        .and(warp::post())
        .and(json_body::<ApiSiteCreateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::create_site)
}

/// PATCH /site/:id with JSON body
pub fn site_update(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("site" / String)
        .and(warp::patch())
        .and(json_body::<ApiSiteUpdateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::update_site)
}

/// DELETE /site/:id
pub fn site_delete(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("site" / String)
        .and(warp::delete())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::delete_site)
}

/// GET /site/:id/page?offset=3&limit=5
pub fn site_pages_list(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("site" / String / "page")
        .and(warp::get())
        .and(warp::query::<ListOptions>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::list_site_pages)
}

/// POST /site/:id/page with JSON body
pub fn site_page_create(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("site")
        .and(warp::post())
        .and(json_body::<ApiSitePageCreateSpec>())
        .and(with_session(db_pool.clone()))
        .and(with_db(db_pool))
        .and_then(handlers::create_site_page)
}
