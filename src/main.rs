#[macro_use]
extern crate lazy_static;
pub mod handlers;
mod response;
mod torrent;
mod proxys;
use proxys::Proxys;
use handlers::rarbg::{get_torrnets,get_all_torrents};
use actix_web::{get, post, put, delete, web, App, HttpRequest, HttpResponse, HttpServer, Responder, ResponseError, middleware};
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::body::BoxBody;

use serde::{Serialize, Deserialize};
use torrent::REQWEST_CLIENT;



use std::fmt::Display;
use std::sync::Mutex;
use std::{env, io};



#[actix_rt::main]
async fn main() -> io::Result<()> {   
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            // enable logger - always register actix-web Logger middleware last
            .wrap(middleware::Logger::default())
            // register HTTP requests handlers
            .service(get_torrnets)
            .service(get_all_torrents)
            .service(handlers::fitgirl::get_torrnets)
            .service(handlers::x1337::get_torrnets)
            .service(handlers::dodi::get_torrnets)
    })
    .bind("127.0.0.1:8088")?
    .run()
    .await
}
