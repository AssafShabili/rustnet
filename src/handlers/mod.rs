pub mod rarbg;
pub mod dodi;
pub mod fitgirl;
pub mod x1337;
pub mod galaxy;

use crate::torrent::{Torrent, Torrents};
use actix_web::{HttpResponse, web::Path};
use async_trait::async_trait;


/// Generic struct to represent all of the torrents Handlers
pub struct TorrentHandler<'a> {
    pub search_value: &'a str,
}


/// Generic trait to represent the extract_info function
#[async_trait]
pub trait ExtractInfo {
    async fn extract_info(
       &self
    ) -> Result<Torrents, Box<dyn std::error::Error>>;
}


#[async_trait]
pub trait GetTorrents {
    async fn get_torrnets(&self,path: Path<(String, usize)>) -> HttpResponse;
}






/// Return a ['String'] type that is the html of the requested url
pub async fn get_request(url:reqwest::Url) -> Result<String,reqwest::Error> {
    Ok(crate::torrent::REQWEST_CLIENT
        .get(url)
        .send()
        .await?
        .text()
        .await?)
}




pub enum TorrnetError{
    reqwestError,
    selectError,
}

impl From<reqwest::Error> for TorrnetError {
    fn from(value: reqwest::Error) -> Self {
        TorrnetError::reqwestError
    }
}

impl From<std::io::Error> for TorrnetError {
    fn from(value: std::io::Error) -> Self {
        TorrnetError::selectError
    }
}