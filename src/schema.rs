// @generated automatically by Diesel CLI.

diesel::table! {
    memo (id) {
        id -> Integer,
        content -> Nullable<Text>,
        language -> Nullable<Text>,
        source_type -> Nullable<Text>,
        result_type -> Nullable<Text>,
    }
}
