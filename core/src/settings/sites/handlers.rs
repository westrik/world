use std::convert::Infallible;
use warp::http::StatusCode;
use warp::Rejection;

use crate::auth::models::session::Session;
use crate::db::{get_conn, DbPool};
use crate::errors::ApiError;
use crate::settings::sites::models::site::{ApiSite, ApiSiteCreateSpec, ApiSiteUpdateSpec, Site};
use crate::settings::sites::models::site_page::{ApiSitePage, SitePage};
use crate::utils::api_task::run_api_task;
use crate::utils::list_options::ListOptions;

#[derive(Serialize)]
pub struct GetSitesResponse {
    error: Option<String>,
    sites: Option<Vec<ApiSite>>,
}

#[derive(Serialize)]
pub struct UpdateSiteResponse {
    error: Option<String>,
    site: Option<ApiSite>,
}

#[derive(Serialize)]
pub struct GetSitePagesResponse {
    error: Option<String>,
    #[serde(rename = "sitePages")]
    site_pages: Option<Vec<ApiSitePage>>,
}

fn run_get_sites(session: Session, pool: &DbPool) -> Result<Vec<Site>, ApiError> {
    Ok(Site::find_all_for_user(&get_conn(&pool).unwrap(), session)?)
}

pub async fn list_sites(
    opts: ListOptions,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("list_sites: opts={:?}", opts);
    let sites = run_api_task(move || run_get_sites(session, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetSitesResponse {
            error: None,
            sites: Some(sites.iter().map(ApiSite::from).collect()),
        }),
        StatusCode::OK,
    ))
}

fn run_create_site(session: Session, description: String, pool: &DbPool) -> Result<Site, ApiError> {
    Ok(Site::create(
        &get_conn(&pool).unwrap(),
        session,
        description,
    )?)
}

pub async fn create_site(
    new_site: ApiSiteCreateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("create_site new_site={:?}", new_site);
    let site =
        run_api_task(move || run_create_site(session, new_site.description, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateSiteResponse {
            error: None,
            site: Some(ApiSite::from(&site)),
        }),
        StatusCode::OK,
    ))
}

fn run_update_site(
    session: Session,
    api_id: String,
    spec: ApiSiteUpdateSpec,
    pool: &DbPool,
) -> Result<Site, ApiError> {
    Ok(Site::update(
        &get_conn(&pool).unwrap(),
        session,
        api_id,
        spec,
    )?)
}

pub async fn update_site(
    api_id: String,
    spec: ApiSiteUpdateSpec,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("update_site: api_id={}, spec={:?}", api_id, spec);
    let site = run_api_task(move || run_update_site(session, api_id, spec, &db_pool)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&UpdateSiteResponse {
            error: None,
            site: Some(ApiSite::from(&site)),
        }),
        StatusCode::OK,
    ))
}

pub async fn delete_site(
    api_id: String,
    _session: Session,
    _db_pool: DbPool,
) -> Result<impl warp::Reply, Infallible> {
    debug!("delete_site: api_id={}", api_id);
    Ok(StatusCode::NO_CONTENT)
}

fn run_get_site_pages(
    session: Session,
    pool: &DbPool,
    site_api_id: String,
) -> Result<Vec<SitePage>, ApiError> {
    Ok(SitePage::find_all_for_site(
        &get_conn(&pool).unwrap(),
        session,
        site_api_id,
    )?)
}

pub async fn list_site_pages(
    site_api_id: String,
    opts: ListOptions,
    session: Session,
    db_pool: DbPool,
) -> Result<impl warp::Reply, Rejection> {
    debug!("list_site_pages: opts={:?}", opts);
    let pages = run_api_task(move || run_get_site_pages(session, &db_pool, site_api_id)).await?;
    Ok(warp::reply::with_status(
        warp::reply::json(&GetSitePagesResponse {
            error: None,
            site_pages: Some(pages.iter().map(ApiSitePage::from).collect()),
        }),
        StatusCode::OK,
    ))
}
