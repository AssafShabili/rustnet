use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{get, web::Path, HttpResponse};
use select::predicate::{Attr, Class, Name,Not};
use select::{document::Document, predicate::Predicate};

// url of the fitgirl website. ? should I implement a more secure way in order to avoid a fake websites ? i dunno
pub const URL: &str = "https://1337xx.to/";

async fn extract_info(search_value: &str, page: usize) -> Result<Torrents, reqwest::Error> {
    let mut torrents: Vec<Torrent> = Vec::new();
    let html = REQWEST_CLIENT
        .get(format!("{}search/{}/{}", URL, search_value, page))
        .send()
        .await?
        .text()
        .await?;

    //println!("{:?}",html);
    let document = Document::from_read(html.as_bytes()).unwrap();

    //TODO: return error if 'search value' is not right

    let tr = document
        .find(Name("tr"))
        .enumerate();

    for (_,td) in tr {
        //println!("{:#?}",td.1.html());
        let name = td.find(Name("td").and(Attr("class","coll-1 name"))).enumerate();

        for f in name {
            println!("{:?}",f.1);
        }

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
