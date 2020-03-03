use crate::db::{get_conn, DbPool};
use crate::fixtures::{
    get_test_user, TEST_USER_EMAIL_ADDRESS, TEST_USER_FULL_NAME, TEST_USER_PASSWORD,
};
use chrono::{Duration, Utc};
use westrikworld_core::auth::models::session::Session;
use westrikworld_core::auth::models::user::User;

#[test_case]
fn test_get_user(pool: DbPool) {
    let conn = get_conn(&pool).unwrap();
    let user = get_test_user(&conn);
    assert_eq!(user.email_address, TEST_USER_EMAIL_ADDRESS);
    assert_eq!(user.full_name, Some(TEST_USER_FULL_NAME.to_string()));
}

#[test_case]
fn test_sign_in(pool: DbPool) {
    let conn = get_conn(&pool).unwrap();
    let user = User::find(TEST_USER_EMAIL_ADDRESS, TEST_USER_PASSWORD, &conn).unwrap();
    let session = Session::create(&conn, &user).unwrap();
    let two_weeks_from_now = Utc::now() + Duration::days(14);

    assert_eq!(user.full_name, Some(TEST_USER_FULL_NAME.to_string()));
    assert_eq!(session.user_id, user.id);
    assert_eq!((session.expires_at - two_weeks_from_now).num_minutes(), 0);
}
