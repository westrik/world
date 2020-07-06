table! {
    block_versions (id) {
        id -> Int4,
        block_id -> Int4,
        created_at -> Timestamptz,
        content -> Jsonb,
        position -> Int4,
        note_version_id -> Int4,
        updated_at -> Timestamptz,
    }
}

table! {
    blocks (id) {
        id -> Int4,
        api_id -> Varchar,
        note_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    jobs (id) {
        id -> Int4,
        api_id -> Varchar,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        status -> Varchar,
        job_type -> Varchar,
        payload -> Nullable<Jsonb>,
        user_id -> Nullable<Int4>,
    }
}

table! {
    note_versions (id) {
        id -> Int4,
        api_id -> Varchar,
        note_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

table! {
    notes (id) {
        id -> Int4,
        api_id -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
    }
}

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
        api_id -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        completed_at -> Nullable<Timestamptz>,
        description -> Text,
        sibling_id -> Nullable<Int4>,
        parent_id -> Nullable<Int4>,
        is_collapsed -> Bool,
        block_id -> Int4,
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

joinable!(block_versions -> note_versions (note_version_id));
joinable!(blocks -> notes (note_id));
joinable!(jobs -> users (user_id));
joinable!(note_versions -> notes (note_id));
joinable!(notes -> users (user_id));
joinable!(sessions -> users (user_id));
joinable!(tasks -> blocks (block_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    block_versions,
    blocks,
    jobs,
    note_versions,
    notes,
    sessions,
    tasks,
    users,
);