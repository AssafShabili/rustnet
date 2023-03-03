use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{get, web::Path, HttpResponse};
use select::predicate::{Attr, Class, Name};
use select::{document::Document, predicate::Predicate};

// url of the fitgirl website. ? should I implement a more secure way in order to avoid a fake websites ? i dunno
pub const URL: &str = "https://dodi-repacks.download/";

// url - https://dodi-repacks.download/page/2/?s=gta+


async fn extract_info(search_value: &str, page: usize) -> Result<Torrents, reqwest::Error> {
    let mut torrents: Vec<Torrent> = Vec::new();
    let html = REQWEST_CLIENT
        .get(format!("{}page/{}/?s={}", URL,page,search_value))
        .send()
        .await?
        .text()
        .await?;

    println!("{:?}",html);

    let document = Document::from_read(html.as_bytes()).unwrap();
    let ts = Torrents {
        results: vec![torrents],
    };
    Ok(ts)
}



#[get("/dodi/torrents/{search}/{page}")]
pub async fn get_torrnets(path: Path<(String, usize)>) -> HttpResponse {
    let torrents = extract_info(&path.0, path.1).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}