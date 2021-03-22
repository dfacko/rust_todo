use crate::bcrypt::*;
use crate::diesel::prelude::*;
use crate::error::*;
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

#[derive(Clone, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "todo_item"]
pub struct TodoItemNew {
    pub list_id: i32,
    pub task: String,
    pub finished: bool,
}

#[derive(Debug, Queryable, Serialize, Clone)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub pword: String,
}

#[derive(Clone, Debug, Insertable, Serialize, Deserialize)]
#[table_name = "user_"]
pub struct UserNew {
    pub username: String,
    pub pword: String,
}

impl TodoList {
    pub fn get_list_by_id(conn: &PgConnection, list_id: i32) -> Result<TodoList, Error> {
        use crate::schema::todo_list::dsl::*;

        let list = todo_list.find(list_id).get_result::<TodoList>(conn)?;
        Ok(list)
    }

    pub async fn get_all_lists(conn: &PgConnection) -> Result<Vec<TodoList>, Error> {
        use crate::schema::todo_list::dsl::*;
        todo_list
            .load::<TodoList>(conn)
            .map_err(|err| Error::from(err))
    }

    pub fn create_list<'a>(conn: &PgConnection, list: TodoListNew<'a>) -> Result<TodoList, Error> {
        diesel::insert_into(todo_list::table)
            .values(&list)
            .get_result::<TodoList>(conn)
            .map_err(|err| Error::from(err))
    }

    pub fn delete_list(
        // returns 0 if no rows are deleted
        conn: &PgConnection,
        delete_id: i32,
    ) -> Result<usize, Error> {
        use crate::schema::todo_list::dsl::*;
        match diesel::delete(todo_list.filter(id.eq(delete_id))).execute(conn) {
            Ok(num_deleted_rows) => {
                println!("{:?}", num_deleted_rows);
                Ok(num_deleted_rows)
            }
            Err(error) => {
                println!("{:?}", error);
                Err(Error::from(error))
            }
        }
    }
}

impl TodoItem {
    pub fn create_item(conn: &PgConnection, item: TodoItemNew) -> Result<TodoItem, Error> {
        use crate::schema::todo_item::dsl::*;
        diesel::insert_into(todo_item)
            .values(&item)
            .get_result(conn)
            .map_err(|err| Error::from(err))
    }
    pub fn items_from_list(conn: &PgConnection, some_list_id: i32) -> Result<Vec<TodoItem>, Error> {
        use crate::schema::todo_item::dsl::*;
        match todo_item
            .filter(list_id.eq(some_list_id))
            .load::<TodoItem>(conn)
        {
            Ok(item_list) => Ok(item_list),
            Err(error) => Err(Error::from(error)),
        }
    }

    pub fn delete_item(
        // returns 0 if no rows are deleted
        conn: &PgConnection,
        delete_id: i32,
    ) -> Result<usize, Error> {
        use crate::schema::todo_item::dsl::*;
        match diesel::delete(todo_item.filter(id.eq(delete_id))).execute(conn) {
            Ok(num_deleted_rows) => {
                println!("{:?}", num_deleted_rows);
                Ok(num_deleted_rows)
            }
            Err(error) => {
                println!("{:?}", error);
                Err(Error::from(error))
            }
        }
    }

    pub fn _get_item_by_id(conn: &PgConnection, item_id: i32) -> Result<Vec<TodoItem>, Error> {
        use crate::schema::todo_item::dsl::*;

        match todo_item.find(item_id).load::<TodoItem>(conn) {
            Ok(item) => Ok(item),
            Err(error) => Err(Error::from(error)),
        }
    }

    pub fn check(conn: &PgConnection, item_id: i32) -> Result<TodoItem, Error> {
        use crate::schema::todo_item::dsl::*;

        diesel::update(todo_item.filter(id.eq(item_id)))
            .set(finished.eq(true))
            .get_result(conn)
            .map_err(|err| Error::from(err))
    }

    pub fn uncheck(conn: &PgConnection, item_id: i32) -> Result<TodoItem, Error> {
        use crate::schema::todo_item::dsl::*;

        diesel::update(todo_item.filter(id.eq(item_id)))
            .set(finished.eq(false))
            .get_result(conn)
            .map_err(|err| Error::from(err))
    }

    pub fn return_ok(_conn: &PgConnection) -> Result<bool, Error> {
        Ok(true)
    }
}

impl User {
    pub fn create_user(conn: &PgConnection, mut user: UserNew) -> Result<User, Error> {
        user.pword = match bcrypt::hash(user.pword, 4) {
            Ok(hash) => hash,

            Err(err) => return Err(Error::from(err)),
        };
        use crate::schema::user_::dsl::*;
        diesel::insert_into(user_)
            .values(user)
            .get_result(conn)
            .map_err(|err| Error::from(err))
    }

    pub fn login(
        conn: &PgConnection,
        login_username: String,
        password: String,
    ) -> Result<Vec<User>, Error> {
        use crate::schema::user_::dsl::*;

        user_
            .find(id)
            .filter(username.eq(login_username))
            .load::<User>(conn)
            .map(|user| {
                if user.len() > 0 && check_pass(&user[0].pword, &password) {
                    user
                } else {
                    vec![]
                }
            })
            .map_err(|err| Error::from(err))
    }

    pub fn user_lists(conn: &PgConnection, user: User) -> Result<Vec<TodoList>, Error> {
        use crate::schema::todo_list::dsl::*;
        todo_list
            .filter(user_id.eq(user.id))
            .get_results(conn)
            .map_err(|err| Error::from(err))
    }
}

fn check_pass(user_pass: &str, input_pass: &str) -> bool {
    println!("checking pass");
    match verify(input_pass, user_pass) {
        Ok(ok) => ok,
        Err(err) => {
            println!("error {:?}", err);
            false
        }
    }
}
