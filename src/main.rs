#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate futures;

mod db;
mod json_validation;
mod models;
mod schema;
mod todoitem_routes;
mod todolist_routes;
mod user_routes;
use self::diesel::prelude::*;
use actix_service::Service;
use futures::future::FutureExt;

use actix_web::{web, App, HttpResponse, HttpServer};
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
            .wrap_fn(|req, srv| {
                println!("middleware{:?}", req);
                let auth_header = req.headers().get("Authorization");
                match auth_header {
                    Some(header) => {
                        println!("gut {:?}", header);
                    }
                    None => {
                        println!("no auth header");
                    }
                }
                srv.call(req).map(|res| res)
            })
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
            .route(
                "/checkitem/{item_id}",
                web::get().to(todoitem_routes::check_item),
            )
            .route(
                "/uncheckitem/{item_id}",
                web::get().to(todoitem_routes::uncheck_item),
            )
            .route("/register", web::post().to(user_routes::create_user))
            .route("/login", web::post().to(user_routes::login))
            .route("/test", web::get().to(todoitem_routes::return_ok))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
