use crate::db::PgPool;
use warp::Filter;

pub fn with_db(
    db_pool: PgPool,
) -> impl Filter<Extract = (PgPool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db_pool.clone())
}

pub fn with_session_token() -> impl Filter<Extract = (String,), Error = warp::Rejection> + Clone {
    warp::any()
        .and(warp::header("authorization"))
        .map(|token: String| token)
}

pub fn json_body<T: Send + serde::de::DeserializeOwned>(
) -> impl Filter<Extract = (T,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json::<T>())
}
