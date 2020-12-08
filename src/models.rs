use crate::diesel::prelude::*;
use crate::schema::*;
#[derive(Debug, Queryable, Serialize, Clone)]
pub struct TodoList {
    pub id: i32,
    pub title: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "todo_list"]
pub struct TodoListNew<'a> {
    pub title: &'a str,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct TodoItem {
    pub id: i32,
    pub list_id: i32,
    pub task: String,
    pub finished: bool,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "todo_item"]
pub struct TodoItemNew<'a> {
    pub list_id: i32,
    pub task: &'a str,
    pub finished: bool,
}

impl TodoList {
    pub fn get_list_by_id(
        conn: &PgConnection,
        list_id: i32,
    ) -> Result<TodoList, diesel::result::Error> {
        use crate::schema::todo_list::dsl::*;

        todo_list.find(list_id).get_result::<TodoList>(conn)
    }

    pub async fn get_all_lists(
        conn: &PgConnection,
    ) -> Result<Vec<TodoList>, diesel::result::Error> {
        use crate::schema::todo_list::dsl::*;
        todo_list.load::<TodoList>(conn)
    }

    pub fn create_list<'a>(
        conn: &PgConnection,
        list: TodoListNew<'a>,
    ) -> Result<TodoList, diesel::result::Error> {
        diesel::insert_into(todo_list::table)
            .values(&list)
            .get_result::<TodoList>(conn)
    }

    pub fn delete_list(
        // returns 0 if no rows are deleted
        conn: &PgConnection,
        delete_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::todo_list::dsl::*;
        match diesel::delete(todo_list.filter(id.eq(delete_id))).execute(conn) {
            Ok(num_deleted_rows) => {
                println!("{:?}", num_deleted_rows);
                Ok(num_deleted_rows)
            }
            Err(error) => {
                println!("{:?}", error);
                Err(error)
            }
        }
    }
}

impl TodoItem {
    pub fn create_item(
        conn: &PgConnection,
        item: TodoItemNew,
    ) -> Result<TodoItem, diesel::result::Error> {
        use crate::schema::todo_item::dsl::*;
        diesel::insert_into(todo_item)
            .values(&item)
            .get_result(conn)
    }
    pub fn items_from_list(
        conn: &PgConnection,
        some_list_id: i32,
    ) -> Result<Vec<TodoItem>, diesel::result::Error> {
        use crate::schema::todo_item::dsl::*;
        match todo_item
            .filter(list_id.eq(some_list_id))
            .load::<TodoItem>(conn)
        {
            Ok(item_list) => Ok(item_list),
            Err(error) => Err(error),
        }
    }

    pub fn delete_item(
        // returns 0 if no rows are deleted
        conn: &PgConnection,
        delete_id: i32,
    ) -> Result<usize, diesel::result::Error> {
        use crate::schema::todo_item::dsl::*;
        match diesel::delete(todo_item.filter(id.eq(delete_id))).execute(conn) {
            Ok(num_deleted_rows) => {
                println!("{:?}", num_deleted_rows);
                Ok(num_deleted_rows)
            }
            Err(error) => {
                println!("{:?}", error);
                Err(error)
            }
        }
    }

    pub fn get_item_by_id(
        conn: &PgConnection,
        item_id: i32,
    ) -> Result<Vec<TodoItem>, diesel::result::Error> {
        use crate::schema::todo_item::dsl::*;

        match todo_item.find(item_id).load::<TodoItem>(conn) {
            Ok(item) => Ok(item),
            Err(error) => Err(error),
        }
    }
}
