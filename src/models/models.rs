use crate::diesel::prelude::*;
use crate::schema::*;
#[derive(Debug, Queryable, Serialize, Clone)]
pub struct TodoList {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "todo_list"]
pub struct TodoListNew<'a> {
    pub title: &'a str,
    pub user_id: i32,
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

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pword: String,
}

#[derive(Debug, Insertable, Serialize, Deserialize)]
#[table_name = "user_"]
pub struct UserNew {
    pub username: String,
    pub pword: String,
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

    pub fn _get_item_by_id(
        conn: &PgConnection,
        item_id: i32,
    ) -> Result<Vec<TodoItem>, diesel::result::Error> {
        use crate::schema::todo_item::dsl::*;

        match todo_item.find(item_id).load::<TodoItem>(conn) {
            Ok(item) => Ok(item),
            Err(error) => Err(error),
        }
    }

    pub fn check(conn: &PgConnection, item_id: i32) -> Result<TodoItem, diesel::result::Error> {
        use crate::schema::todo_item::dsl::*;

        diesel::update(todo_item.filter(id.eq(item_id)))
            .set(finished.eq(true))
            .get_result(conn)
    }

    pub fn uncheck(conn: &PgConnection, item_id: i32) -> Result<TodoItem, diesel::result::Error> {
        use crate::schema::todo_item::dsl::*;

        diesel::update(todo_item.filter(id.eq(item_id)))
            .set(finished.eq(false))
            .get_result(conn)
    }

    pub fn return_ok(_conn: &PgConnection) -> Result<bool, diesel::result::Error> {
        Ok(true)
    }
}

impl User {
    pub fn create_user(
        conn: &PgConnection,
        mut user: UserNew,
    ) -> Result<User, diesel::result::Error> {
        let hash = user.pword;
        user.pword = bcrypt::hash(hash, 2).unwrap();
        use crate::schema::user_::dsl::*;
        diesel::insert_into(user_).values(user).get_result(conn)
    }

    pub fn login(
        conn: &PgConnection,
        login_username: String,
        password: String,
    ) -> Result<Vec<User>, diesel::result::Error> {
        use crate::schema::user_::dsl::*;

        user_
            .find(id)
            .filter(username.eq(login_username))
            .load::<User>(conn)
            .map(|user| {
                if user.len() > 0 && check_pass(&user[0].pword, &password) {
                    println!("user found, passwords match");
                    user
                } else {
                    vec![]
                }
            })
    }

    pub fn user_lists(
        conn: &PgConnection,
        user: User,
    ) -> Result<Vec<TodoList>, diesel::result::Error> {
        use crate::schema::todo_list::dsl::*;
        todo_list.filter(user_id.eq(user.id)).get_results(conn)
    }
}

fn check_pass(user_pass: &str, input_pass: &str) -> bool {
    if user_pass == input_pass {
        true
    } else {
        false
    }
}
