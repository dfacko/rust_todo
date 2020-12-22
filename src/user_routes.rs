use super::db::*;
use super::json_validation;
use super::models::*;
use actix_web::{web, Error, FromRequest, HttpRequest, HttpResponse, Responder};
use serde_json::{json, to_string, Value};

pub async fn create_user(conn: web::Data<Pool>, newuser: web::Json<Value>) -> impl Responder {
    let data: String =
        match json_validation::validate(&newuser, vec!["username|string", "pword|string"]) {
            Some(error) => return Ok(HttpResponse::UnprocessableEntity().json(error)),
            None => to_string(&newuser.into_inner()).unwrap(),
        };
    let insertable_user: UserNew = serde_json::from_str(&data).unwrap();
    println!("Created new user");
    User::create_user(&conn.get().unwrap(), insertable_user)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|_| HttpResponse::InternalServerError().finish())
}

pub async fn login(conn: web::Data<Pool>, newuser: web::Json<Value>) -> impl Responder {
    let data: String =
        match json_validation::validate(&newuser, vec!["username|string", "pword|string"]) {
            Some(error) => return Ok(HttpResponse::UnprocessableEntity().json(error)),
            None => to_string(&newuser.into_inner()).unwrap(),
        };
    let insertable_user: UserNew = serde_json::from_str(&data).unwrap();
    User::login(
        &conn.get().unwrap(),
        insertable_user.username.to_string(),
        insertable_user.pword.to_string(),
    )
    .map(|user| {
        if user.len() > 0 {
            HttpResponse::Ok().json(json!({
                "message":"sucessful log in",
                "user":user[0]
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message":"incorrect credentials"
            }))
        }
    })
    .map_err(|_| HttpResponse::InternalServerError().finish())
}
