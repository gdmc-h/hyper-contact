// @generated automatically by Diesel CLI.

diesel::table! {
    contacts (id) {
        id -> Integer,
        first -> Text,
        last -> Text,
        phone -> Text,
        email -> Text,
    }
}

diesel::table! {
    todos (id) {
        id -> Nullable<Binary>,
        title -> Text,
        done -> Bool,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    contacts,
    todos,
);
