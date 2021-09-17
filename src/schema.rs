table! {
    todo_item (id) {
        id -> Uuid,
        list_id -> Uuid,
        task -> Varchar,
        finished -> Bool,
    }
}

table! {
    todo_list (id) {
        id -> Uuid,
        user_id -> Uuid,
        title -> Varchar,
    }
}

table! {
    user_ (id) {
        id -> Uuid,
        username -> Varchar,
        pword -> Varchar,
    }
}

joinable!(todo_item -> todo_list (list_id));
joinable!(todo_list -> user_ (user_id));

allow_tables_to_appear_in_same_query!(
    todo_item,
    todo_list,
    user_,
);
