use crate::db::*;
use crate::error::*;
use crate::json_validation;
use crate::models::models::*;
use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, to_string, Value};

pub async fn additem(
    conn: web::Data<Pool>,
    newitem: web::Json<Value>,
    req: web::HttpRequest,
) -> impl Responder {
    let data: TodoItemNew = match json_validation::validate::<TodoItemNew>(
        newitem,
        vec!["username|string", "password|string"],
    ) {
        Err(err) => return Err(err.to_response()),
        Ok(data) => data,
    };

    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    match User::user_lists(&conn.get().unwrap(), user) {
        Ok(lists) => {
            for list in lists {
                if list.id == data.list_id {
                    return TodoItem::create_item(&conn.get().unwrap(), data)
                        .map(|item| HttpResponse::Ok().json(item))
                        .map_err(|err| err.to_response());
                }
            }
            return Err(Error::throw("Unauthorized", Some("Not your list")).to_response());
        }

        Err(err) => return Err(err.to_response()),
    };
}

pub async fn items_from_list(
    conn: web::Data<Pool>,
    list_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;
    match User::user_lists(&conn.get().unwrap(), user.clone()) {
        Ok(lists) => {
            for list in lists {
                if list.id == list_id.clone() && list.user_id == user.id {
                    return TodoItem::items_from_list(&conn.get().unwrap(), list_id.into_inner())
                        .map(|items| HttpResponse::Ok().json(items))
                        .map_err(|err| err.to_response());
                }
            }
            return Err(Error::throw("Unauthorized", Some("Cannot edit that list")).to_response());
        }

        Err(err) => {
            return Err(err.to_response());
        }
    };
}

pub async fn delete_item(
    conn: web::Data<Pool>,
    item_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    if user.id == item_id.clone() {
        TodoItem::delete_item(&conn.get().unwrap(), item_id.into_inner())
            .map(|item| {
                HttpResponse::Ok().json(json!({
                    "message": format!("{} item has been deleted", item)
                }))
            })
            .map_err(|err| err.to_response())
    } else {
        Err(HttpResponse::NotFound().json(json!({
            "message":"item not found"
        })))
    }
}

pub async fn check_item(
    conn: web::Data<Pool>,
    item_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    if user.id == item_id.clone() {
        TodoItem::check(&conn.get().unwrap(), item_id.into_inner())
            .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
            .map_err(|_| HttpResponse::InternalServerError().finish())
    } else {
        Err(HttpResponse::NotFound().json(json!({
            "message":"item not found"
        })))
    }
}

pub async fn uncheck_item(
    conn: web::Data<Pool>,
    item_id: web::Path<i32>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;
    if user.id == item_id.clone() {
        TodoItem::uncheck(&conn.get().unwrap(), item_id.into_inner())
            .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
            .map_err(|_| HttpResponse::InternalServerError().finish())
    } else {
        Err(HttpResponse::NotFound().json(json!({
            "message":"item not found"
        })))
    }
}

pub async fn return_ok(conn: web::Data<Pool>, newitem: web::Json<Value>) -> impl Responder {
    let _data: TodoItemNew = match json_validation::validate::<TodoItemNew>(
        newitem,
        vec!["username|string", "password|string"],
    ) {
        Err(err) => return Err(err.to_response()),
        Ok(data) => data,
    };
    TodoItem::return_ok(&conn.get().unwrap())
        .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}
