use crate::torrent::{Torrent, Torrents, REQWEST_CLIENT};
use actix_web::{get, web::Path, HttpResponse};
use select::predicate::{Attr, Class, Name};
use futures::future::join_all;
use select::{document::Document, predicate::Predicate};
use std::borrow::Cow;

struct TorrentGalaxy<'a> {
    search:&'a str
}

impl TorrentGalaxy {
    const url: Cow<'static, str> = Cow::Borrowed("https://url.of.website/search=");
    fn new()
}