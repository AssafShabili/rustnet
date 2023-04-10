use crate::{response::Response, proxys::Proxys};
use serde::{Deserialize, Serialize};
use reqwest::{Client, Error, Proxy};

lazy_static! {
    // pub static ref REQWEST_CLIENT_PROXYS:Client = {
    //     let mut client = Client::builder();
    //     for proxy in Proxys::new().unwrap().data {
    //         client = client.proxy(
    //             Proxy::http(format!("{}://{}:{}",proxy.protocols[0],proxy.ip,proxy.port)).unwrap()
    //         );
    //     }
    //     return client.user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.106 Safari/537.36")
    //     .build().unwrap();
    // };

    pub static ref REQWEST_CLIENT: Client = {
        let proxy = Proxy::http("http://195.154.67.61:3128").unwrap();
        let client = Client::builder()
                    .proxy(proxy)
                    .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.106 Safari/537.36")
                    .build().unwrap();
        return client;
    };
}



pub type Torrents = Response<Vec<Torrent>>;
#[derive(Debug,Clone, Deserialize, Serialize, Default)]
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

    pub fn set_name(&mut self, new_name: String) {
        self.name = new_name;
    }
    pub fn set_category(&mut self, new_category: String) {
        self.category = new_category;
    }
    pub fn set_date_uploaded(&mut self, new_date_uploaded: String) {
        self.date_uploaded = new_date_uploaded;
    }

    pub fn set_size(&mut self, new_size: String) {
        self.size = new_size;
    }

    pub fn set_seeders(&mut self, new_seeders: usize) {
        self.seeders = new_seeders;
    }
    pub fn set_leechers(&mut self, new_leechers: usize) {
        self.leechers = new_leechers;
    }
    pub fn set_uploaded_by(&mut self, new_uploaded_by: String) {
        self.uploaded_by = new_uploaded_by;
    }
    pub fn set_url(&mut self, url: String) {
        self.url = url;
    }

    pub fn set_magnet_link(&mut self, new_magnet_link: String) {
        self.magnet_url = new_magnet_link;
    }

    pub fn magnet_link_mut(&mut self) -> &mut String {
        &mut self.magnet_url
    }

    pub fn url_mut(&mut self) -> &mut String {
        &mut self.url
    }
}
