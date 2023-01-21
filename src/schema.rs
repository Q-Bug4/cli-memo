// @generated automatically by Diesel CLI.

diesel::table! {
    memos (id) {
        id -> Integer,
        name -> Text,
        content -> Text,
        language -> Text,
        source_type -> Text,
        result_type -> Text,
    }
}
