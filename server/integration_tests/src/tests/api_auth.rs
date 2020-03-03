use crate::db::DbPool;
use westrikworld_core::auth::filters::sign_up;
use westrikworld_core::auth::models::user::ApiUserCreateSpec;

#[test_case]
fn test_sign_up(pool: DbPool) {
    let sign_up_filter = sign_up(pool);
    let sign_up_request = ApiUserCreateSpec {
        emailAddress: "bernard@westrikworld.com".to_string(),
        fullName: Some("Bernard Lowe".to_string()),
        password: "abc123".to_string(),
    };
    // TODO: use tokio to run this?
    // let response = warp::test::request()
    //     .path("/sign-up")
    //     .method("POST")
    //     .header("accept", "application/json")
    //     .json(&sign_up_request)
    //     .reply(&sign_up_filter)
    //     .await;
    // println!("{:?}", response);
}
