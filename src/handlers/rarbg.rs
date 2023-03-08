use crate::response::Response;
use crate::torrent::{Torrent, Torrents,REQWEST_CLIENT};
use actix_web::{
    get,
    middleware::Logger,
    web::{self, Path},
    HttpResponse,
};
use env_logger::Env;
use hyper::client::connect::dns::GaiResolver;
use hyper::client::HttpConnector;
use hyper_rustls::ConfigBuilderExt;
use hyper_rustls::HttpsConnector;

use native_tls::TlsConnector;
use select::document::Document;
use select::predicate::{Class, Name, Predicate};

use reqwest::{Client, Error, Proxy};
pub const URL: &str = "https://rargb.to/search/";



async fn extract_info(search_value: &str, page: usize) -> Result<Torrents, reqwest::Error> {
    let html = REQWEST_CLIENT
        .get(
            format!("{}{}//?search={}", URL, page, search_value)
        )
        .send()
        .await?
        .text()
        .await?;
    let document = Document::from_read(html.as_bytes()).unwrap();
    let table = document.find(Class("lista2t")).next().unwrap();

    let mut torrents: Vec<Torrent> = Vec::new();

    //TODO: maybe needs to refactor this, this is not the best
    for node in table.find(Class("lista2")) {
        //td == line in the table
        let mut td = node.find(Name("td")).take(8);

        let mut torrent = Torrent::default();

        td.next(); //<kirby falling into the void meme here>

        let name_td = td.next().unwrap();
        // adding a comments
        torrent.set_name(String::from(name_td.text().trim()));
        torrent.set_category(String::from(td.next().unwrap().text().trim()));
        torrent.set_date_uploaded(String::from(td.next().unwrap().text().trim()));
        torrent.set_size(String::from(td.next().unwrap().text().trim()));
        torrent.set_seeders(
            String::from(td.next().unwrap().text().trim())
                .parse()
                .unwrap(),
        );
        torrent.set_leechers(
            String::from(td.next().unwrap().text().trim())
                .parse()
                .unwrap(),
        );
        torrent.set_uploaded_by(String::from(td.next().unwrap().text().trim()));

        let uri = name_td
            .find(Name("a"))
            .next()
            .unwrap()
            .attr("href")
            .unwrap_or_else(|| "");

        if uri == "" {
            torrent.set_url(String::from("[Error] - couldn't find the torrent url."))
        } else {
            torrent.set_url(format!("https://rargb.to{}", uri));
        }

        

        // adding the finished torrent struct to the vector.
        torrents.push(torrent);
    }
    let ts = Torrents {
        results: vec![torrents],
    };
    Ok(ts)
}

#[get("/rarbg/torrents/{search}/{page}")]
pub async fn get_torrnets(path: Path<(String, usize)>) -> HttpResponse {
    
    let torrents = extract_info(&path.0, path.1).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}
#[get("/rarbg/torrents/{search}")]
pub async fn get_all_torrents(path: Path<String>) -> HttpResponse {
    let torrents = extract_info(&path, 1).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}
