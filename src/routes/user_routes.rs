use crate::bcrypt::*;
use crate::db::*;
use crate::error::*;
use crate::json_validation;
use crate::jwt::*;
use crate::models::models::*;
use actix_web::{web, HttpResponse, Responder};
use serde_json::{json, to_string, Value};

pub async fn create_user(conn: web::Data<Pool>, newuser: web::Json<Value>) -> impl Responder {
    let data: String =
        match json_validation::validate(&newuser, vec!["username|string", "password|string"]) {
            Err(err) => return Err(err.to_response()),
            Ok(_) => to_string(&newuser.into_inner()).map_err(|e| Error::from(e).to_response())?,
        };
    let insertable_user: UserNew =
        serde_json::from_str(&data).map_err(|e| Error::from(e).to_response())?;
    User::create_user(&conn.get().unwrap(), insertable_user)
        .map(|user| HttpResponse::Ok().json(user))
        .map_err(|err| err.to_response())
}

pub async fn login(conn: web::Data<Pool>, newuser: web::Json<Value>) -> impl Responder {
    let data: String = match json_validation::validate(&newuser, vec![]) {
        Err(err) => return Err(err.to_response()),
        Ok(_) => to_string(&newuser.into_inner()).map_err(|e| Error::from(e).to_response())?,
    };
    let insertable_user: UserNew =
        serde_json::from_str(&data).map_err(|err| Error::from(err).to_response())?;
    User::login(
        &conn.get().unwrap(),
        insertable_user.username.to_string(),
        insertable_user.pword.to_string(),
    )
    .map(|mut user| {
        if user.len() > 0 {
            let jwt = generate(user[0].clone());
            HttpResponse::Ok().json(json!({
                "message":"sucessful log in",
                "user":user.pop(),
                "jwt":jwt
            }))
        } else {
            HttpResponse::Ok().json(json!({
                "message":"incorrect credentials"
            }))
        }
    })
    .map_err(|err| err.to_response())
}

pub async fn my_lists(conn: web::Data<Pool>, req: web::HttpRequest) -> impl Responder {
    // should be the same as code commented above
    let user = req.extensions_mut().remove::<User>().ok_or_else(|| {
        return Error::throw("Unauthorized", Some("User not found in request")).to_response();
    })?;
    User::user_lists(&conn.get().unwrap(), user)
        .map(|user_list| {
            if user_list.len() > 0 {
                HttpResponse::Ok().json(json!({
                    "message":"your lists",
                    "lists":user_list
                }))
            } else {
                Error::from(json!({
                    "message":"you have no lists"
                }))
                .to_response()
            }
        })
        .map_err(|err| err.to_response())
}
