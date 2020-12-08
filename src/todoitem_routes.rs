use super::db::*;
use super::models::*;
use super::validation;
use actix_web::{web, HttpResponse, Responder};
use serde_json::*;

#[derive(Debug, serde::Deserialize)]
pub struct TodoItemJson {
    pub list_id: i32,
    pub task: String,
    pub finished: bool,
}

pub async fn additem(conn: web::Data<Pool>, newitem: web::Json<Value>) -> impl Responder {
    let data: String = match validation::validate(&newitem, vec!["finished", "list_id", "task"]) {
        Some(error) => return Ok(HttpResponse::InternalServerError().json(error)),
        None => to_string(&newitem.into_inner()).unwrap(),
    };
    let insertable_item: TodoItemNew = serde_json::from_str(&data).unwrap();

    TodoItem::create_item(&conn.get().unwrap(), insertable_item)
        .map(|item| HttpResponse::Ok().json(item))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn items_from_list(conn: web::Data<Pool>, list_id: web::Path<i32>) -> impl Responder {
    TodoItem::items_from_list(&conn.get().unwrap(), list_id.into_inner())
        .map(|items| HttpResponse::Ok().json(items))
        .map_err(|_| HttpResponse::NotFound().finish())
}

pub async fn delete_item(conn: web::Data<Pool>, item_id: web::Path<i32>) -> impl Responder {
    TodoItem::delete_item(&conn.get().unwrap(), item_id.into_inner())
        .map(|item| {
            HttpResponse::Ok().json(json!({
                "message": format!("{} item has been deleted", item)
            }))
        })
        .map_err(|_| HttpResponse::NotFound().finish())
}
