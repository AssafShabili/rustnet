use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{
    get,
    middleware::Logger,
    web::{self, Path},
    HttpResponse,
};
use reqwest::{Client, Error, Proxy};
use select::document::Document;
use select::predicate::{Attr, Class, Name, Text};
use serde::__private::doc;

pub const URL: &str = "https://fitgirl-repacks.site/";

async fn extract_info(search_value: &str, page: usize) -> Result<Torrents, reqwest::Error> {
    //println!("{}", format!("{}page/{}/?s={}", URL, page, search_value));

    let mut torrents: Vec<Torrent> = Vec::new();
    let html = REQWEST_CLIENT
        .get(format!("{}page/{}/?s={}", URL, page, search_value))
        .send()
        .await?
        .text()
        .await?;

    let document = Document::from_read(html.as_bytes()).unwrap();

    //TODO: return error if 'search value' is not right

    let links = document.find(Attr("rel", "bookmark")).enumerate();

    for link in links.step_by(2) {
        let search_link = link.1.attr("href").unwrap();
        let mut torrent = Torrent::default();
        let search_value_html = REQWEST_CLIENT.get(search_link).send().await?.text().await?;
        let search_document = Document::from_read(search_value_html.as_bytes()).unwrap();
        let name = search_document.find(Class("entry-title")).next().unwrap();

        torrent.set_name(String::from(name.text().trim()));
        torrent.set_category(String::from("Games"));

        let date = search_document.find(Class("entry-date")).next().unwrap();
        torrent.set_date_uploaded(String::from(date.text().trim()));
        let size = search_document.find(Name("strong")).nth(4).unwrap().text();
        torrent.set_size(size);
        let magnet = search_document
            .find(Name("a"))
            .find(
                //very very bad. but we need to find the <a> tag that contain the magnet like.
                |&x| x.attr("href").unwrap().contains("magnet:?"),
            )
            .unwrap();
        torrent.set_magnet_link(String::from(magnet.attr("href").unwrap().trim()));

        torrent.set_uploaded_by(String::from("FITGIRL"));
        torrent.set_url(String::from(search_link));
        torrents.push(torrent);
    }
    let ts = Torrents {
        results: vec![torrents],
    };
    Ok(ts)
}

#[get("/fitgirl/torrents/{search}/{page}")]
pub async fn get_torrnets(path: Path<(String, usize)>) -> HttpResponse {
    let formatted_search = path.0.replace(" ", "+");
    let torrents = extract_info(&formatted_search, path.1).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}
