// @generated automatically by Diesel CLI.

diesel::table! {
    categories (id) {
        id -> Nullable<Integer>,
        title -> Text,
    }
}

diesel::table! {
    games (id) {
        id -> Nullable<Integer>,
        category_id -> Integer,
        title -> Text,
        stars -> Integer,
    }
}

diesel::joinable!(games -> categories (category_id));

diesel::allow_tables_to_appear_in_same_query!(categories, games,);
