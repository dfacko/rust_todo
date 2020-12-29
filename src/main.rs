#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate futures;

mod databases;
mod error;
mod middleware;
mod models;
mod routes;
mod schema;
mod validation;
extern crate bcrypt;
use self::diesel::prelude::*;
use actix_service::Service;
use actix_web::{web, App, HttpServer};
use databases::*;
use diesel::r2d2::{self, ConnectionManager};
use futures::future::FutureExt;
use middleware::*;
use routes::{todolist_routes, user_routes};
use validation::*;

#[macro_use]
extern crate serde_derive;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let pool = db::establish_connection();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap_fn(|req, srv| {
                println!("prvi");
                srv.call(req).map(|res| res)
            })
            .wrap_fn(|req, srv| {
                println!("drugi");
                srv.call(req).map(|res| res)
            })
            //.wrap(simplemiddleware::SayHi)
            .wrap(authorize::CheckLogin)
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
            .route("/additem", web::post().to(routes::todoitem_routes::additem))
            .route(
                "/listitems/{list_id}",
                web::get().to(routes::todoitem_routes::items_from_list),
            )
            .route(
                "/deleteitem/{item_id}",
                web::delete().to(routes::todoitem_routes::delete_item),
            )
            .route(
                "/checkitem/{item_id}",
                web::get().to(routes::todoitem_routes::check_item),
            )
            .route(
                "/uncheckitem/{item_id}",
                web::get().to(routes::todoitem_routes::uncheck_item),
            )
            .route("/register", web::post().to(user_routes::create_user))
            .route("/login", web::post().to(user_routes::login))
            .route("/myLists", web::get().to(user_routes::my_lists))
            .route("/test", web::get().to(routes::todoitem_routes::return_ok))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
