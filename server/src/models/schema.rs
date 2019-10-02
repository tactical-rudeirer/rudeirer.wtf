table! {
    news (id) {
        id -> Int4,
        title -> Nullable<Text>,
        content -> Nullable<Text>,
        date -> Nullable<Date>,
        author_id -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        username -> Text,
        password -> Text,
        displayname -> Nullable<Text>,
        role -> Nullable<Text>,
    }
}

allow_tables_to_appear_in_same_query!(
    news,
    users,
);
