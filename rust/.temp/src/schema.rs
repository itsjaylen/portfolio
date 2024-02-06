// @generated automatically by Diesel CLI.

diesel::table! {
    tasks (id) {
        id -> Nullable<Integer>,
        title -> Text,
        description -> Nullable<Text>,
        created_at -> Nullable<Timestamp>,
        completed -> Bool,
        completed_at -> Nullable<Timestamp>,
    }
}
