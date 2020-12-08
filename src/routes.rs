use crate::models;
use crate::Pool;
use actix_web::{get, web, Error, HttpResponse, Responder};
use anyhow::Result;
use diesel::dsl::insert_into;
use diesel::RunQueryDsl;

pub async fn home() -> Result<HttpResponse, Error> {
    web::HttpResponse::Ok().await
}

#[get("/readlist")]
pub async fn read_lists(pool: web::Data<Pool>, req_body: String) -> impl Responder {
    println!("read_lists");
    //println!("{}", req_body);
    match models::TodoList::get_all_lists(&pool.get().unwrap()).await {
        Ok(lists) => {
            println!("{:?}", lists);
            HttpResponse::Ok().json(req_body).await
        }
        Err(error) => {
            println!("{:?}", error);
            HttpResponse::InternalServerError().await.into()
        }
    }
}

pub async fn add_list(
    pool: web::Data<Pool>,
    new_list: web::Json<models::TodoListJson>,
) -> impl Responder {
    match models::TodoList::create_list(&pool.get().unwrap(), &new_list.title).await {
        Ok(todo_list) => {
            println!("{:?}", todo_list);
            web::HttpResponse::Ok().json(todo_list).await
        }
        Err(error) => {
            println!("{:?}", error);
            web::HttpResponse::NotFound().body("{message: Error}").await
        }
    }
}

pub async fn delete_list(pool: web::Data<Pool>) -> impl Responder {
    /*match models::TodoList::delete_list(&pool.get().unwrap(), *id).await {
        Ok(someresponse) => web::HttpResponse::Ok().await,
        Err(someerror) => web::HttpResponse::Ok().await,
    }*/
    web::HttpResponse::Ok().await
}

/*pub async fn add_item_to_list(
    pool: web::Data<Pool>,
    new_item: web::Json<models::TodoItemJson>,
) -> impl Responder {
    match models::TodoItem::add_item_to_list(&pool.get().unwrap(), new_item) {
        Ok(someresponse) => web::HttpResponse::Ok().await,
        Err(someerror) => web::HttpResponse::Ok().await,
    }
}*/
