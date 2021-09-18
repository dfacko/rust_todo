use crate::db::*;
use crate::error::*;
use crate::models::models::*;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use uuid::Uuid;

pub async fn lists(conn: web::Data<Pool>) -> impl Responder {
    let lists = TodoList::get_all_lists(&conn.get().unwrap()).await;
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
    let title: String = newlist
        .into_inner()
        .title
        .ok_or_else(|| HttpResponse::BadRequest().finish())
        .map(|list| list.to_string())?;

    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    let newlist = TodoListNew {
        title,
        user_id: user.id,
    };

    TodoList::create_list(&conn.get().unwrap(), newlist)
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn list_by_id(
    conn: web::Data<Pool>,
    path: web::Path<Uuid>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    let user_lists =
        TodoList::user_lists(&conn.get().unwrap(), user.id).map_err(|e| e.to_response())?;

    if !user_lists.iter().any(|list| list.user_id == user.id) {
        return Err(HttpResponse::Unauthorized().json(json!({
            "message":" Not your list"
        })));
    }

    TodoList::get_list_by_id(&conn.get().unwrap(), path.into_inner())
        .map(|list| HttpResponse::Ok().json(list))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn delete_list(
    conn: web::Data<Pool>,
    path: web::Path<Uuid>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;
    let list_id = match TodoList::get_list_by_id(&conn.get().unwrap(), path.clone()) {
        Ok(list) => {
            if list.user_id != user.id {
                return Ok(HttpResponse::Ok().json(json!({
                    "message":"cannot delete"
                })));
            } else {
                list.id
            }
        }
        Err(_) => {
            println!("no list found");
            return Ok(HttpResponse::Unauthorized().json(json!({
                "message":"cannot delete"
            })));
        }
    };
    TodoList::delete_list(&conn.get().unwrap(), list_id)
        .map(|item| {
            HttpResponse::Ok().json(json!({
                "message": format!("{} item has been removed", item)
            }))
        })
        .map_err(|_| HttpResponse::InternalServerError().finish())
}
