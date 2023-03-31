// @generated automatically by Diesel CLI.

diesel::table! {
    meows (url) {
        number -> Nullable<Int4>,
        url -> Text,
    }
}
