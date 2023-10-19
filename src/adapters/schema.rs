// @generated automatically by Diesel CLI.

diesel::table! {
    channel_permissions (id) {
        id -> Int4,
        user_id -> Int4,
        channel_id -> Int4,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    channels (id) {
        id -> Int4,
        name -> Text,
        description -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    credentials (id) {
        id -> Int4,
        user_id -> Int4,
        password -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        channel_id -> Int4,
        user_id -> Int4,
        content -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        username -> Text,
        email -> Text,
        created_at -> Timestamptz,
    }
}

diesel::joinable!(channel_permissions -> channels (channel_id));
diesel::joinable!(channel_permissions -> users (user_id));
diesel::joinable!(credentials -> users (user_id));
diesel::joinable!(messages -> channels (channel_id));
diesel::joinable!(messages -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    channel_permissions,
    channels,
    credentials,
    messages,
    users,
);
