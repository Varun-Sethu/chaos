use diesel_derive_enum::DbEnum;

#[derive(Debug, DbEnum)]
pub enum ApplicationStatus {
    Pending,
    Rejected,
    Success,
}

#[derive(Debug, DbEnum, PartialEq)]
pub enum AdminLevel {
    ReadOnly,
    Director,
    Admin,
}

table! {
    answers (id) {
        id -> Int4,
        application_id -> Int4,
        question_id -> Int4,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::schema::ApplicationStatusMapping;

    applications (id) {
        id -> Int4,
        user_id -> Int4,
        role_id -> Int4,
        status -> ApplicationStatusMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    campaigns (id) {
        id -> Int4,
        organisation_id -> Int4,
        name -> Text,
        cover_image -> Nullable<Text>,
        description -> Text,
        starts_at -> Timestamp,
        ends_at -> Timestamp,
        draft -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    comments (id) {
        id -> Int4,
        application_id -> Int4,
        commenter_user_id -> Int4,
        description -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::database::schema::AdminLevelMapping;

    organisation_users (id) {
        id -> Int4,
        user_id -> Int4,
        organisation_id -> Int4,
        admin_level -> AdminLevelMapping,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    organisations (id) {
        id -> Int4,
        name -> Text,
        logo -> Nullable<Text>,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    questions (id) {
        id -> Int4,
        role_id -> Int4,
        title -> Text,
        description -> Nullable<Text>,
        max_bytes -> Int4,
        required -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    ratings (id) {
        id -> Int4,
        application_id -> Int4,
        rater_user_id -> Int4,
        rating -> Int4,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    roles (id) {
        id -> Int4,
        campaign_id -> Int4,
        name -> Text,
        description -> Nullable<Text>,
        min_available -> Int4,
        max_available -> Int4,
        finalised -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Text,
        zid -> Text,
        display_name -> Text,
        degree_name -> Text,
        degree_starting_year -> Int4,
        superuser -> Bool,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

joinable!(answers -> applications (application_id));
joinable!(answers -> questions (question_id));
joinable!(applications -> roles (role_id));
joinable!(applications -> users (user_id));
joinable!(campaigns -> organisations (organisation_id));
joinable!(comments -> applications (application_id));
joinable!(comments -> users (commenter_user_id));
joinable!(organisation_users -> organisations (organisation_id));
joinable!(organisation_users -> users (user_id));
joinable!(questions -> roles (role_id));
joinable!(ratings -> applications (application_id));
joinable!(ratings -> users (rater_user_id));
joinable!(roles -> campaigns (campaign_id));

allow_tables_to_appear_in_same_query!(
    answers,
    applications,
    campaigns,
    comments,
    organisation_users,
    organisations,
    questions,
    ratings,
    roles,
    users,
);