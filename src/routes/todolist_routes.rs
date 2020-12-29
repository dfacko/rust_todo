use crate::db::*;
use crate::models::models::*;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

pub async fn lists(conn: web::Data<Pool> /*req: web::HttpRequest*/) -> impl Responder {
    let lists = TodoList::get_all_lists(&conn.get().unwrap()).await;
    /*let user = req.extensions_mut().remove::<User>().unwrap();
    println!("ovo je user iz req: {:?}", user);*/
    match lists {
        Ok(lists) => HttpResponse::build(StatusCode::OK).json(lists),
        Err(error) => {
            println!("error:{:?}", error);
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("{message: nemoze}")
        }
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct TodoListJson {
    pub title: Option<String>,
}

pub async fn addlist(
    conn: web::Data<Pool>,
    newlist: web::Json<TodoListJson>,
    req: web::HttpRequest,
) -> impl Responder {
    let title: String = match &newlist.title {
        Some(title) => title.to_string(),
        None => return Ok(HttpResponse::BadRequest().finish()),
    };
    let user = req.extensions_mut().remove::<User>().unwrap();
    let newlist = TodoListNew {
        title: &title,
        user_id: user.id,
    };

    TodoList::create_list(&conn.get().unwrap(), newlist)
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn list_by_id(conn: web::Data<Pool>, path: web::Path<i32>) -> impl Responder {
    /*match TodoList::get_list_by_id(&conn.get().unwrap(), path.into_inner()).await {
        Ok(list) => HttpResponse::build(StatusCode::OK).json(list),
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                .body("{message: something went wrong when reading from db}")
        }
    }*/

    TodoList::get_list_by_id(&conn.get().unwrap(), path.into_inner())
        .map(|list| HttpResponse::Ok().json(list))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn delete_list(
    conn: web::Data<Pool>,
    path: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().unwrap();
    println!("got user");
    match TodoList::get_list_by_id(&conn.get().unwrap(), path.clone()) {
        Ok(list) => {
            if list.id != user.id {
                return Ok(HttpResponse::Ok().json(json!({
                    "message":"cannot delete"
                })));
            }
        }
        Err(_) => {
            println!("no list found");
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };
    TodoList::delete_list(&conn.get().unwrap(), path.into_inner())
        .map(|item| {
            HttpResponse::Ok().json(json!({
                "message": format!("{} item has been removed", item)
            }))
        })
        .map_err(|_| HttpResponse::InternalServerError().finish())
}
