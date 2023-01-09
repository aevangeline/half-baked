// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        name -> Text,
        description -> Nullable<Text>,
    }
}

diesel::table! {
    events (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        headline -> Text,
        start_time -> Text,
        end_time -> Text,
        description -> Nullable<Text>,
        category_id -> Nullable<Integer>,
        external_source_id -> Nullable<Integer>,
    }
}

diesel::table! {
    externalsource (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        typ -> Text,
        access_token -> Nullable<Text>,
        user_name -> Nullable<Text>,
        source_uri -> Text,
    }
}

diesel::table! {
    todos (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        headline -> Text,
        goal_time -> Nullable<Text>,
        priority -> Nullable<Integer>,
        category_id -> Nullable<Integer>,
        external_source_id -> Nullable<Integer>,
    }
}

diesel::table! {
    userexperiencesettings (id) {
        id -> Nullable<Integer>,
        user_id -> Integer,
        time_zone -> Text,
    }
}

diesel::table! {
    users (id) {
        id -> Nullable<Integer>,
        given_name -> Text,
        last_name -> Nullable<Text>,
        honorific -> Nullable<Text>,
    }
}

diesel::joinable!(categories -> users (user_id));
diesel::joinable!(events -> categories (category_id));
diesel::joinable!(events -> externalsource (external_source_id));
diesel::joinable!(events -> users (user_id));
diesel::joinable!(todos -> categories (category_id));
diesel::joinable!(todos -> externalsource (external_source_id));
diesel::joinable!(todos -> users (user_id));
diesel::joinable!(userexperiencesettings -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    categories,
    events,
    externalsource,
    todos,
    userexperiencesettings,
    users,
);
