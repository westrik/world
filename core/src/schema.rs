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
    media_item_versions (id) {
        id -> Int4,
        api_id -> Varchar,
        user_id -> Int4,
        media_item_id -> Int4,
        created_at -> Timestamptz,
        version_type -> Varchar,
        asset_url -> Nullable<Varchar>,
        asset_file_size_bytes -> Nullable<Int8>,
        asset_data -> Nullable<Jsonb>,
    }
}

table! {
    media_items (id) {
        id -> Int4,
        api_id -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        name -> Varchar,
        presigned_upload_url -> Nullable<Varchar>,
        uploaded_file_name -> Nullable<Varchar>,
        uploaded_file_size_bytes -> Nullable<Int8>,
    }
}

table! {
    note_versions (id) {
        id -> Int4,
        api_id -> Varchar,
        note_id -> Int4,
        created_at -> Timestamptz,
        content -> Jsonb,
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
    site_pages (id) {
        id -> Int4,
        api_id -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        site_id -> Int4,
        note_id -> Int4,
        note_version_id -> Int4,
        path -> Text,
        published -> Bool,
    }
}

table! {
    sites (id) {
        id -> Int4,
        api_id -> Varchar,
        user_id -> Int4,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
        title -> Varchar,
        bucket_domain_name -> Nullable<Varchar>,
        bucket_access_key_id -> Nullable<Varchar>,
        bucket_access_key_secret -> Nullable<Varchar>,
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
        api_id -> Varchar,
    }
}

joinable!(jobs -> users (user_id));
joinable!(media_item_versions -> media_items (media_item_id));
joinable!(media_item_versions -> users (user_id));
joinable!(media_items -> users (user_id));
joinable!(note_versions -> notes (note_id));
joinable!(notes -> users (user_id));
joinable!(sessions -> users (user_id));
joinable!(site_pages -> note_versions (note_version_id));
joinable!(site_pages -> notes (note_id));
joinable!(site_pages -> sites (site_id));
joinable!(site_pages -> users (user_id));
joinable!(sites -> users (user_id));
joinable!(tasks -> users (user_id));

allow_tables_to_appear_in_same_query!(
    jobs,
    media_item_versions,
    media_items,
    note_versions,
    notes,
    sessions,
    site_pages,
    sites,
    tasks,
    users,
);
