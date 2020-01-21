table! {
    sessions (token) {
        user_id -> Int4,
        token -> Text,
        created_at -> Timestamptz,
        expires_at -> Timestamptz,
    }
}

table! {
    tasks (id) {
        id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    users (id) {
        id -> Int4,
        email_address -> Text,
        full_name -> Nullable<Text>,
        password_hash -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

joinable!(sessions -> users (user_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(sessions, tasks, users,);
