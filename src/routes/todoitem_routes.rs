use crate::db::*;
use crate::error::*;
use crate::json_validation;
use crate::models::models::*;
use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, to_string, Value};
use uuid::Uuid;

pub async fn additem(
    conn: web::Data<Pool>,
    newitem: web::Json<Value>,
    req: web::HttpRequest,
) -> impl Responder {
    let data: TodoItemNew = match json_validation::validate::<TodoItemNew>(
        newitem,
        vec!["task|string", "list_id|string"],
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
    list_id: web::Path<Uuid>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;
    match User::user_lists(&conn.get().unwrap(), user.clone()) {
        Ok(lists) => {
            for list in lists {
                if list.id == *list_id && list.user_id == user.id {
                    return TodoItem::items_from_list(&conn.get().unwrap(), *list_id)
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
    item_id: web::Path<Uuid>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    let item =
        TodoItem::_get_item_by_id(&conn.get().unwrap(), *item_id).map_err(|e| e.to_response())?;

    let user_lists_ids = TodoList::user_lists(&conn.get().unwrap(), user.id)
        .map_err(|e| e.to_response())?
        .into_iter()
        .map(|item| item.id)
        .collect::<Vec<Uuid>>();

    if !user_lists_ids.contains(&item.list_id) {
        return Err(HttpResponse::Unauthorized().json(json!({
            "message":" not your item"
        })));
    }

    TodoItem::delete_item(&conn.get().unwrap(), *item_id, user_lists_ids)
        .map(|item| {
            HttpResponse::Ok().json(json!({
                "message": format!("{} item has been deleted", item)
            }))
        })
        .map_err(|err| err.to_response())
}

pub async fn check_item(
    conn: web::Data<Pool>,
    item_id: web::Path<Uuid>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    let item =
        TodoItem::_get_item_by_id(&conn.get().unwrap(), *item_id).map_err(|e| e.to_response())?;

    let user_lists_ids = TodoList::user_lists(&conn.get().unwrap(), user.id)
        .map_err(|e| e.to_response())?
        .into_iter()
        .map(|item| item.id)
        .collect::<Vec<Uuid>>();

    if !user_lists_ids.contains(&item.list_id) {
        return Err(HttpResponse::Unauthorized().json(json!({
            "message":" not your item"
        })));
    }

    TodoItem::check(&conn.get().unwrap(), *item_id)
        .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
        .map_err(|_| {
            HttpResponse::NotFound().json(json!({
                "message":"item not found"
            }))
        })
}

pub async fn uncheck_item(
    conn: web::Data<Pool>,
    item_id: web::Path<Uuid>,
    req: web::HttpRequest,
) -> impl Responder {
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        Error::throw("Unauthorized", Some("User not found in request")).to_response()
    })?;

    let item =
        TodoItem::_get_item_by_id(&conn.get().unwrap(), *item_id).map_err(|e| e.to_response())?;

    let user_lists_ids = TodoList::user_lists(&conn.get().unwrap(), user.id)
        .map_err(|e| e.to_response())?
        .into_iter()
        .map(|item| item.id)
        .collect::<Vec<Uuid>>();

    if !user_lists_ids.contains(&item.list_id) {
        return Err(HttpResponse::Unauthorized().json(json!({
            "message":" not your item"
        })));
    }

    TodoItem::uncheck(&conn.get().unwrap(), *item_id)
        .map(|item| HttpResponse::Ok().json(json!({ "updated item": item })))
        .map_err(|_| {
            HttpResponse::NotFound().json(json!({
                "message":"item not found"
            }))
        })
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
