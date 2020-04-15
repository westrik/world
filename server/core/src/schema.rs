table! {
    block_versions (id) {
        id -> Int4,
        block_id -> Int4,
        created_at -> Timestamptz,
        content -> Jsonb,
    }
}

table! {
    blocks (id) {
        id -> Int4,
        api_id -> Varchar,
        user_id -> Int4,
        note_id -> Nullable<Int4>,
        position -> Int4,
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

joinable!(blocks -> notes (note_id));
joinable!(blocks -> users (user_id));
joinable!(jobs -> users (user_id));
joinable!(notes -> users (user_id));
joinable!(sessions -> users (user_id));
joinable!(tasks -> blocks (block_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(block_versions, blocks, jobs, notes, sessions, tasks, users,);
