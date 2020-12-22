use warp::http::StatusCode;
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::db::DbPool;
use crate::errors::ApiError;
use crate::links::models::link::LinkSummary;
use crate::utils::api_task::run_api_task;
use crate::utils::list_options::ListOptions;

#[derive(Serialize)]
pub struct GetLinksResponse {
    error: Option<String>,
    links: Option<Vec<LinkSummary>>,
}

fn run_list_inbound_links_for_note(
    _session: Session,
    _pool: &DbPool,
    _options: ListOptions,
) -> Result<Vec<LinkSummary>, ApiError> {
    unimplemented!()
}

pub async fn list_inbound_links_for_note(
    session: Session,
    db_pool: DbPool,
    options: ListOptions,
) -> Result<impl warp::Reply, Rejection> {
    debug!("list_inbound_links_for_note: opts={:?}", &options);
    let links =
        run_api_task(move || run_list_inbound_links_for_note(session, &db_pool, options)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetLinksResponse {
            error: None,
            links: Some(links),
        }),
        StatusCode::OK,
    ))
}
