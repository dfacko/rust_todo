use super::db::*;
use super::json_validation;
use super::models::*;
use actix_web::{web, HttpResponse, Responder};
use serde_json::*;

pub async fn additem(
    conn: web::Data<Pool>,
    newitem: web::Json<Value>,
    req: web::HttpRequest,
) -> impl Responder {
    let data: String = match json_validation::validate(
        &newitem,
        vec!["finished|bool", "list_id|int", "task|string"],
    ) {
        Some(error) => return Ok(HttpResponse::UnprocessableEntity().json(error)),
        None => to_string(&newitem.into_inner()).unwrap(),
    };
    let insertable_item: TodoItemNew = serde_json::from_str(&data).unwrap();
    let user = req.extensions_mut().remove::<User>().unwrap();
    match User::user_lists(&conn.get().unwrap(), user) {
        Ok(lists) => {
            for list in lists {
                if list.id == insertable_item.list_id {
                    return TodoItem::create_item(&conn.get().unwrap(), insertable_item)
                        .map(|item| HttpResponse::Ok().json(item))
                        .map_err(|_| {
                            HttpResponse::InternalServerError().json(json!({
                                "message":"failed to insert into list"
                            }))
                        });
                }
            }
            return Ok(HttpResponse::Unauthorized().json(json!({
                "message":"Cannot edit that list"
            })));
        }

        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "message":"failed to get user lists"
            })))
        }
    };
}

pub async fn items_from_list(
    conn: web::Data<Pool>,
    list_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().unwrap();
    match User::user_lists(&conn.get().unwrap(), user.clone()) {
        Ok(lists) => {
            for list in lists {
                if list.id == list_id.clone() && list.user_id == user.id {
                    return TodoItem::items_from_list(&conn.get().unwrap(), list_id.into_inner())
                        .map(|items| HttpResponse::Ok().json(items))
                        .map_err(|_| HttpResponse::NotFound().finish());
                }
            }
            return Ok(HttpResponse::Unauthorized().json(json!({
                "message":"Cannot edit that list"
            })));
        }

        Err(_) => {
            return Ok(HttpResponse::InternalServerError().json(json!({
                "message":"failed to get user lists"
            })))
        }
    };
}

pub async fn delete_item(
    conn: web::Data<Pool>,
    item_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    TodoItem::delete_item(&conn.get().unwrap(), item_id.into_inner())
        .map(|item| {
            HttpResponse::Ok().json(json!({
                "message": format!("{} item has been deleted", item)
            }))
        })
        .map_err(|_| HttpResponse::NotFound().finish())
}

pub async fn check_item(
    conn: web::Data<Pool>,
    item_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    TodoItem::check(&conn.get().unwrap(), item_id.into_inner())
        .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn uncheck_item(
    conn: web::Data<Pool>,
    item_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    TodoItem::uncheck(&conn.get().unwrap(), item_id.into_inner())
        .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn return_ok(conn: web::Data<Pool>, newitem: web::Json<Value>) -> impl Responder {
    let data: String = match json_validation::validate(
        &newitem,
        vec!["finished|bool", "list_id|int", "task|string"],
    ) {
        Some(error) => return Ok(HttpResponse::UnprocessableEntity().json(error)),
        None => to_string(&newitem.into_inner()).unwrap(),
    };
    let insertable_item: TodoItemNew = serde_json::from_str(&data).unwrap();
    println!("Created todo item {:?}", insertable_item);
    TodoItem::return_ok(&conn.get().unwrap())
        .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}
