use crate::{response, torrent::REQWEST_CLIENT};
use serde::{Deserialize, Serialize};

const API_URL:&str = "https://proxylist.geonode.com/api/proxy-list?limit=500&page=1&sort_by=responseTime&sort_type=desc&filterUpTime=90&country=DE&speed=fast";

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proxys {
    pub data: Vec<Proxy>,
}
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Proxy {
    pub ip: String,
    pub asn: String,
    #[serde(rename = "created_at")]
    pub created_at: String,
    pub isp: String,
    pub latency: f64,
    pub port: String,
    pub protocols: Vec<String>,
    pub response_time: i64,
    pub speed: i64,
    pub up_time: f64,
}


impl Proxys {
    pub fn new() -> Result<Proxys, Box<dyn std::error::Error>> {
        let response = REQWEST_CLIENT.get(API_URL).send();
        let fut = futures::executor::block_on(response).unwrap().text();
        let json_string = futures::executor::block_on(fut).unwrap();
        let proxys: Proxys = serde_json::from_str(&json_string)?;
        Ok(proxys)
    }
}
