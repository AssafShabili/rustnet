use crate::response::Response;
use serde::{Deserialize, Serialize};

pub type Torrents = Response<Vec<Torrent>>;
#[derive(Debug, Deserialize, Serialize, Default)]
pub struct Torrent {
    pub name: String,
    pub category: String,
    pub date_uploaded: String,
    pub size: String,
    pub seeders: usize,
    pub leechers: usize,
    pub uploaded_by: String,
    pub url: String,
    pub magnet_url: String,
}
impl Torrent {
    pub fn new(
        name: String,
        category: String,
        date_uploaded: String,
        size: String,
        seeders: usize,
        leechers: usize,
        uploaded_by: String,
        url: String,
        magnet_url: String,
    ) -> Self {
        Self {
            name,
            category,
            date_uploaded,
            size,
            seeders,
            leechers,
            uploaded_by,
            url,
            magnet_url,
        }
    }
}