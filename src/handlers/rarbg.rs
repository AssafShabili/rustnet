use std::thread;

use crate::response::Response;
use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{
    get,
    middleware::Logger,
    web::{self, Path},
    HttpResponse,
};
use env_logger::Env;
use futures::{stream, StreamExt};
use futures::future::join_all;
use reqwest::blocking;

use select::document::Document;
use select::predicate::{And, Attr, Child, Class, Name, Predicate, Text};

use reqwest::{Client, Error, Proxy};
pub const URL: &str = "https://rargb.to/";

async fn extract_info(
    search_value: &str,
    page: usize,
) -> Result<Torrents, Box<dyn std::error::Error>> {
    let html = REQWEST_CLIENT
        .get(format!("{}search/{}//?search={}", URL, page, search_value))
        .send()
        .await?
        .text()
        .await?;

    let document = Document::from_read(html.as_bytes()).unwrap();

    let tr_list = document.find(Class("lista2")).enumerate();

    let mut torrents: Vec<Torrent> = Vec::new();

    for (_, node) in tr_list {
        let mut torrent = Torrent::default();

        let name_td = node
            .find(Attr("align", "left").child(Attr("title", ())))
            .next()
            .unwrap();
        let torrnet_url = format!("{}{}", URL, name_td.attr("href").unwrap());
        torrent.set_name(format!("{}", name_td.attr("title").unwrap()));
        torrent.set_url(torrnet_url);

        let td_vec = node
            .find(Name("td").and(Attr("align", "center")))
            .collect::<Vec<_>>();

        let category: String = td_vec[0].children().map(|a_child| a_child.text()).collect();

        torrent.set_category(category);
        torrent.set_date_uploaded(td_vec[1].text());
        torrent.set_size(td_vec[2].text());
        torrent.set_seeders(td_vec[3].first_child().unwrap().text().parse().unwrap());
        torrent.set_leechers(td_vec[4].text().parse().unwrap());
        torrent.set_uploaded_by(td_vec[5].text());
        torrents.push(torrent);
    }



    let test = join_all(torrents.clone().into_iter().map(|mut torrent| {
        async move {
            let resp = REQWEST_CLIENT.get(&torrent.url).send().await.unwrap().text().await;
                    match resp {
                        Ok(resp) => {
                            let document_magnet = Document::from_read(resp.as_bytes()).unwrap();
                            let magnet_url = document_magnet
                                .find(Name("a").and(Attr("href", ())))
                                .filter(|a| match a.attr("href") {
                                    Some(x) => x.contains("magnet:?"),
                                    None => false,
                                })
                                .next()
                                .unwrap()
                                .attr("href")
                                .unwrap();
                            String::from(magnet_url)
                            //torrent.set_magnet_link(String::from(magnet_url));
                        },
                        Err(e) => {
                            //torrent.set_magnet_link(String::from("Couldn't get the magnet"));
                            String::from("Couldn't get the magnet")
                        }
                    }
        }
    })).await;


    for i in 0..torrents.len(){
        torrents[i].set_magnet_link(test[i].to_string());
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
