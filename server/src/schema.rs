// @generated automatically by Diesel CLI.

diesel::table! {
    messages (id) {
        id -> Int4,
        fk_user_id -> Nullable<Int4>,
        message_content -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    profiles (id) {
        id -> Int4,
        fk_user_id -> Nullable<Int4>,
        age -> Int4,
        bio -> Nullable<Text>,
        avatar_url -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    profiles_tags (fk_profile_id, fk_tag_id) {
        fk_profile_id -> Int4,
        fk_tag_id -> Int4,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    tags (id) {
        id -> Int4,
        #[max_length = 20]
        tag_name -> Varchar,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 255]
        username -> Varchar,
        email -> Nullable<Text>,
        password_hash -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
    }
}

diesel::joinable!(messages -> users (fk_user_id));
diesel::joinable!(profiles -> users (fk_user_id));
diesel::joinable!(profiles_tags -> profiles (fk_profile_id));
diesel::joinable!(profiles_tags -> tags (fk_tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    profiles,
    profiles_tags,
    tags,
    users,
);
