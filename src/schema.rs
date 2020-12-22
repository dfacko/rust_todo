table! {
    todo_item (id) {
        id -> Int4,
        list_id -> Int4,
        task -> Varchar,
        finished -> Bool,
    }
}

table! {
    todo_list (id) {
        id -> Int4,
        user_id -> Int4,
        title -> Varchar,
    }
}

table! {
    user_ (id) {
        id -> Int4,
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
