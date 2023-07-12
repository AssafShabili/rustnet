use crate::handlers::{ExtractInfo, GetTorrents, TorrentHandler};
use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{get, web::Path, HttpResponse};
use async_trait::async_trait;
use futures::future::join_all;
use select::predicate::{Attr, Class, Name};
use select::{document::Document, predicate::Predicate};
use std::borrow::Cow;
use std::convert::TryInto;

struct TorrentGalaxy<'a> {
    search: &'a str,
}

impl<'a> TorrentGalaxy<'a> {
    pub const WEBSITE_URL: Cow<'static, str> = Cow::Borrowed("https://torrentgalaxy.to/");
    /// Creates a new [`TorrentGalaxy`].
    fn new(search: &'a str) -> Self {
        Self { search }
    }
}

#[async_trait]
impl<'a> ExtractInfo for TorrentGalaxy<'a> {
    async fn extract_info(&self) -> Result<Torrents, Box<dyn std::error::Error>> {
        let mut torrents: Vec<Torrent> = Vec::new();

        let html = REQWEST_CLIENT
            .get(format!(
                "{}torrents.php?search={}&lang=0&nox=2&nowildcard=1#results",
                Self::WEBSITE_URL,
                self.search
            ))
            .send()
            .await?
            .text()
            .await?;
        let document = Document::from_read(html.as_bytes()).unwrap();

        let div_iter = document
            .find(Attr("class", "tgxtablerow txlight"))
            .enumerate();

        for (_, div) in div_iter {
            let name_node = div.find(Attr("class", "txlight")).nth(0).unwrap();

            let url = format!(
                "{}torrents.php?search={}",
                Self::WEBSITE_URL,
                name_node.attr("href").unwrap()
            );

            let name = name_node.attr("title").unwrap();

            let category = div
                .find(Name("a").child(Name("small")))
                .nth(0)
                .unwrap()
                .text();

            let magnet_link = div
                .find(
                    Attr("class", "tgxtablecell collapsehide rounded txlight")
                        .child(Name("a").and(Attr("role", "button"))),
                )
                .nth(0)
                .unwrap()
                .attr("href")
                .unwrap();

            let size = div
                .find(Attr("class", "badge badge-secondary txlight"))
                .nth(0)
                .unwrap()
                .text();

            let mut download_rate_node = div
                .find(Attr("title", "Seeders/Leechers"))
                .nth(0)
                .unwrap()
                .descendants();

            let seeders: usize = download_rate_node.nth(1).unwrap().text().parse().unwrap();
            let leechers: usize = download_rate_node.nth(3).unwrap().text().parse().unwrap();

            let uploaded_by = div
                .find(Attr("class", "username"))
                .nth(0)
                .unwrap()
                .descendants()
                .nth(0)
                .unwrap()
                .text();

            let date_uploaded = div
                .find(Name("div").child(Name("small")))
                .nth(0)
                .unwrap()
                .text();

            let torrnet = Torrent::new(
                name.to_string(),
                category,
                date_uploaded,
                size,
                seeders,
                leechers,
                uploaded_by,
                magnet_link.to_string(),
                url,
            );

            torrents.push(torrnet)
        }

        let ts = Torrents {
            results: vec![torrents],
        };
        Ok(ts)
    }
}

#[get("/torrentgalaxy/torrents/{search}")]
pub async fn get_torrnets(search: Path<String>) -> HttpResponse {
    // let torrents = extract_info(&path.0, path.1).await.unwrap();
    let torrents = TorrentGalaxy::new(&search).extract_info().await.unwrap();
    HttpResponse::Ok()
        .content_type("application/json")
        .json(torrents)
}
