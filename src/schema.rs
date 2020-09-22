table! {
    posts (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        date -> Timestamp,
        reading_time -> Integer,
        summary -> Text,
    }
}

table! {
    projects (id) {
        id -> Integer,
        title -> Text,
        link -> Text,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(posts, projects,);
