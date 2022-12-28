use reqwest::header::{HeaderMap, HeaderName};
use serde_derive::{Deserialize, Serialize};
use std::{path::Path, fs};

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct HeaderConfig {
    headers: Vec<Header>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Header {
    name: String,
    value: String,
}

pub fn get_headers() -> HeaderMap {
    let header_conf = Path::new("resources/headers.toml");
    if fs::read(header_conf).expect("Could not read headers file").is_empty() {
        return HeaderMap::new();
    }

    let cfg: HeaderConfig = confy::load_path(header_conf).expect("Error reading config.");

    let mut headers = HeaderMap::new();
    for header in cfg.headers {
        let formatted_name = header
            .name
            .to_lowercase()
            .chars()
            .map(|x| x as u8)
            .collect::<Vec<u8>>();

        let header_name = match HeaderName::from_lowercase(&formatted_name as &[u8]) {
            Err(e) => panic!("{:?}", e),
            Ok(name) => name,
        };

        let header_value = match header.value.parse() {
            Err(e) => panic!("{:?}", e),
            Ok(value) => value,
        };

        headers.insert(header_name, header_value);
    }

    headers
}
