table! {
    sessions (id) {
        id -> Binary,
        user_id -> Integer,
        created -> Timestamp,
        last_seen -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        name -> Text,
        password -> Text,
    }
}

table! {
    shares (id) {
        id -> Integer,
        name -> Text,
        path -> Text,
        access_level -> Integer,
    }
}

table! {
    user_shares (user_id, share_id) {
        user_id -> Integer,
        share_id -> Integer,
    }
}

allow_tables_to_appear_in_same_query!(shares, user_shares,);
// allow_tables_to_appear_in_same_query!(sessions, users,);
