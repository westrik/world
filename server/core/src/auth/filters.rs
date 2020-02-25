use crate::auth::handlers;
use crate::auth::handlers::SignInRequest;
use crate::auth::models::user::ApiUserCreateSpec;
use crate::db::DbPool;
use crate::routes::{json_body, with_db};
use warp::Filter;

pub fn routes(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    sign_up(db_pool.clone()).or(sign_in(db_pool))
}

/// POST /sign_up with JSON body
pub fn sign_up(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign-up")
        .and(warp::post())
        .and(json_body::<ApiUserCreateSpec>())
        .and(with_db(db_pool))
        .and_then(handlers::sign_up)
}

/// POST /sign-in with JSON body
pub fn sign_in(
    db_pool: DbPool,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign-in")
        .and(warp::post())
        .and(json_body::<SignInRequest>())
        .and(with_db(db_pool))
        .and_then(handlers::sign_in)
}

#[cfg(test)]
pub mod test_authentication {
    use crate::auth::filters::sign_up;
    use crate::auth::models::user::ApiUserCreateSpec;
    use crate::resource_identifier::*;
    use crate::tasks::models::task::TaskCreateSpec;
    use crate::test_utils::db::{connect_to_test_db, get_conn, rollback};
    use crate::test_utils::fixtures::create_test_user;

    #[test]
    fn test_sign_up() {
        async {
            let sign_up_filter = sign_up(connect_to_test_db());
            let sign_up_request = ApiUserCreateSpec {
                emailAddress: "test_sign_up_user@example.com".to_string(),
                fullName: Some("Mr. Test User".to_string()),
                password: "abc123".to_string(),
            };
            let response = warp::test::request()
                .path("/sign-up")
                .method("POST")
                .header("accept", "application/json")
                .json(&sign_up_request)
                .reply(&sign_up_filter)
                .await;
            println!("{:?}", response);
            let pool = connect_to_test_db();
            rollback(&pool);
        };
    }
}
