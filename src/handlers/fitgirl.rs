use super::get_request;
use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{get, web::Path, HttpResponse};
use futures::future::join_all;
use select::predicate::{Attr, Class, Name};
use select::{document::Document, predicate::Predicate};

// url of the fitgirl website. ? should I implement a more secure way in order to avoid a fake websites ? i dunno
pub const URL: &str = "https://fitgirl-repacks.site/";

async fn extract_info(
    search_value: &str,
    page: usize,
) -> Result<Torrents, Box<dyn std::error::Error>> {
    let mut torrents: Vec<Torrent> = Vec::new();
    let url = reqwest::Url::parse(&format!("{}page/{}/?s={}", URL, page, search_value))?;
    let html = get_request(url).await?;
    let document = Document::from_read(html.as_bytes())?;
    let links = document.find(Attr("rel", "bookmark")).enumerate();

    for link in links.step_by(2) {
        let mut torrent = Torrent::default();
        let search_link = link.1.attr("href").unwrap();
        torrent.set_uploaded_by(String::from("FITGIRL"));
        torrent.set_url(String::from(search_link));
        torrents.push(torrent);
    }

    let torrents_vec: Vec<Torrent> = join_all(torrents.clone().into_iter().map(|mut torrent| {
        async move {
            let resp = REQWEST_CLIENT
                .get(&torrent.url)
                .send()
                .await
                .unwrap()
                .text()
                .await;

            match resp {
                Ok(resp) => {
                    let search_document = Document::from_read(resp.as_bytes()).unwrap();
                    let name = search_document.find(Class("entry-title")).next().unwrap();

                    torrent.set_name(String::from(name.text().trim()));
                    torrent.set_category(String::from("Games"));

                    let date = search_document.find(Class("entry-date")).next().unwrap();
                    torrent.set_date_uploaded(String::from(date.text().trim()));

                    let size = search_document
                        .find(Attr("style", "height: 200px; display: block;").child(Name("strong")))
                        .nth(4);
                    match size {
                        Some(s) => {
                            // we found a size;
                            torrent.set_size(s.text());
                        }
                        None => {
                            torrent.set_size(String::from(
                                "[Error] - couldn't extract the search size ...",
                            ));
                        }
                    }

                    let magnet = search_document.find(Name("a")).find(
                        //very very bad. but we need to find the <a> tag that contain the magnet like.
                        |&x| x.attr("href").unwrap().contains("magnet:?"),
                    );
                    match magnet {
                        Some(m) => {
                            torrent.set_magnet_link(String::from(m.attr("href").unwrap().trim()));
                        }
                        None => {
                            torrent.set_magnet_link(String::from(""));
                        }
                    }

                    torrent.set_uploaded_by(String::from("FITGIRL"));
                    return torrent
                }
                Err(_e) => Torrent::default(),
            }
        }
    }))
    .await;

    let ts = Torrents {
        results: vec![torrents_vec],
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
