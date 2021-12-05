table! {
    messages (id) {
        id -> Binary,
        user_id -> Binary,
        room_id -> Binary,
        message -> Varchar,
        send_at -> Datetime,
    }
}

table! {
    rooms (id) {
        id -> Binary,
        name -> Varchar,
        created_at -> Datetime,
    }
}

table! {
    subscribes (id) {
        id -> Binary,
        user_id -> Binary,
        room_id -> Binary,
        subscribe_at -> Datetime,
    }
}

table! {
    users (id) {
        id -> Binary,
        username -> Varchar,
        password -> Varchar,
        created_at -> Datetime,
    }
}

joinable!(messages -> rooms (room_id));
joinable!(messages -> users (user_id));
joinable!(subscribes -> rooms (room_id));
joinable!(subscribes -> users (user_id));

allow_tables_to_appear_in_same_query!(
    messages,
    rooms,
    subscribes,
    users,
);
