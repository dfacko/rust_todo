#[macro_use]
extern crate diesel;
extern crate dotenv;

mod db;
mod models;
mod schema;
mod todoitem_routes;
mod todolist_routes;
mod validation;
use self::diesel::prelude::*;

use actix_web::{web, App, HttpServer};
use diesel::r2d2::{self, ConnectionManager};

#[macro_use]
extern crate serde_derive;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/lists", web::get().to(todolist_routes::lists))
            .route("/addlist", web::post().to(todolist_routes::addlist))
            .route(
                "/listbyid/{list_id}",
                web::delete().to(todolist_routes::delete_list),
            )
            .route(
                "/listbyid/{list_id}",
                web::get().to(todolist_routes::list_by_id),
            )
            .route("/additem", web::post().to(todoitem_routes::additem))
            .route(
                "/listitems/{list_id}",
                web::get().to(todoitem_routes::items_from_list),
            )
            .route(
                "/deleteitem/{item_id}",
                web::delete().to(todoitem_routes::delete_item),
            )
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}