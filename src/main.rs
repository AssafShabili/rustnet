use rustnet::handlers::rarbg::{get_torrnets,get_all_torrents};
use actix_web::{App, HttpServer,middleware};
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
            .service(rustnet::handlers::fitgirl::get_torrnets)
            .service(rustnet::handlers::x1337::get_torrnets)
            .service(rustnet::handlers::dodi::get_torrnets)
            .service(rustnet::handlers::galaxy::get_torrnets)
    })
    .bind("0.0.0.0:443")?
    .run()
    .await
}
