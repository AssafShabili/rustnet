use crate::response::Response;
use std::sync::Arc;
use crate::torrent::{Torrent,Torrents};
use actix_web::{HttpResponse,get, web::{self, Path}, middleware::Logger};
use env_logger::Env;
use native_tls::TlsConnector;
use hyper_rustls::HttpsConnector;
use hyper_rustls::ConfigBuilderExt;
use hyper::client::HttpConnector;
use hyper::client::connect::dns::GaiResolver;


lazy_static! {
    pub static ref HTTPS_CLIENT: hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector<hyper::client::connect::dns::GaiResolver>>, hyper::Body> = {
        
        let tls = rustls::ClientConfig::builder()
        .with_safe_defaults()
        .with_native_roots()
        .with_no_client_auth();
        let https = hyper_rustls::HttpsConnectorBuilder::new()
        .with_tls_config(tls)
        .https_or_http()
        .enable_http1()
        .build();  
        let client :hyper::Client<hyper_rustls::HttpsConnector<hyper::client::HttpConnector<hyper::client::connect::dns::GaiResolver>>, hyper::Body> = hyper::Client::builder().build::<_,hyper::Body>(https);
        return client;
    };

}



#[get("/rarbg/torrents/{search}/{page}")]
pub async fn get_torrnets(path:Path<(String,usize)>) -> HttpResponse {
    println!("{:?} {:?}",path.0,path.1);

    let torrents = Torrents {
        results:vec![
           vec![
            Torrent::default(),
            Torrent::default(),
            Torrent::default()
           ]
        ]
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}
#[get("/rarbg/torrents/{search}")]
pub async fn get_all_torrents(path:Path<String>) -> HttpResponse {
    println!("{:?}",path);

    let torrents = Torrents {
        results:vec![
           vec![
            Torrent::default(),
            Torrent::default(),
            Torrent::default()
           ]
        ]
    };
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}


