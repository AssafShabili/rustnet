use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{get, web::Path, HttpResponse};
use select::predicate::{Attr, Class, Name};
use select::{document::Document, predicate::Predicate};


pub const URL: &str = "https://1337xx.to/";

async fn get_magent_link(url: &str) -> Result<String, reqwest::Error> {
    let html = REQWEST_CLIENT.get(url).send().await?.text().await?;
    let document = Document::from_read(html.as_bytes()).unwrap();
    let magnet = document.find(
        Attr("class","l3426749b3b895e9356348e295596e5f2634c98d8 l0d669aa8b23687a65b2981747a14a1be1174ba2c la1038a02a9e0ee51f6e4be8730ec3edea40279a2 torrentdown2")
    ).next().unwrap().attr("href").unwrap();
    Ok(String::from(magnet))
}

async fn extract_info(search_value: &str, page: usize) -> Result<Torrents, reqwest::Error> {
    let mut torrents: Vec<Torrent> = Vec::new();
    let html = REQWEST_CLIENT
        .get(format!("{}search/{}/{}", URL, search_value, page))
        .send()
        .await?
        .text()
        .await?;
    let document = Document::from_read(html.as_bytes()).unwrap();

    //TODO: return error if 'search value' is not right

    let trs_iter = document.find(Name("tr")).enumerate().skip(1);

    for (_, tr) in trs_iter {
        let torrent_name_node = tr
            .find(Attr("class", "coll-1 name").child(Attr("class", "icon").not()))
            .next()
            .unwrap();
        //let torrent_uri = torrent_name_node.first_child().unwrap().attr("href").unwrap();
        let torrent_name = torrent_name_node.text();
        let x1337_url_torrent = format!("{}{}", URL, torrent_name_node.attr("href").unwrap());

        let seeders: usize = tr
            .find(Attr("class", "coll-2 seeds"))
            .next()
            .unwrap()
            .text()
            .parse()
            .unwrap();
        let leechers: usize = tr
            .find(Attr("class", "coll-3 leeches"))
            .next()
            .unwrap()
            .text()
            .parse()
            .unwrap();
        let date = tr.find(Attr("class", "coll-date")).next().unwrap().text();
        let size = tr
            .find(Attr("class", "coll-4 size mob-uploader"))
            .next()
            .unwrap()
            .text();
        let uploaded_by = tr
            .find(Attr("class", "coll-5 uploader").child(Name("a")))
            .next()
            .unwrap()
            .text();

        let manget_link = get_magent_link(&x1337_url_torrent).await?;

        let t = Torrent::new(
            torrent_name,
            String::from("Games"),
            date,
            size,
            seeders,
            leechers,
            uploaded_by,
            x1337_url_torrent,
            manget_link,
        );
        torrents.push(t);
    }

    let ts = Torrents {
        results: vec![torrents],
    };
    Ok(ts)
}

#[get("/x1337/torrents/{search}/{page}")]
pub async fn get_torrnets(path: Path<(String, usize)>) -> HttpResponse {
    let torrents = extract_info(&path.0, path.1).await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}
