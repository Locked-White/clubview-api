table! {
    use diesel::sql_types::*;
    use crate::models::MessageType;

    messages (id) {
        id -> Uuid,
        author_id -> Text,
        content_type -> Message_type,
        message -> Text,
        channel -> Uuid,
        meta -> Jsonb,
        posted_at -> Nullable<Timestamp>,
    }
}

table! {
    use diesel::sql_types::*;
    use crate::models::MessageType;

    users (id) {
        id -> Int4,
        first_name -> Text,
        last_name -> Text,
        email -> Text,
        created_at -> Timestamp,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
