use crate::writer::write_to_file;
use reqwest::{header::HeaderMap, Error, RequestBuilder, Response, Url};
use serde_json::Value;
use std::{collections::HashMap, path::Path};

pub async fn get(url: Url, headers: HeaderMap, body: String) -> Result<Response, Error> {
    let client = reqwest::Client::new();
    make_request(client.get(url), headers, body).await
}

pub async fn post(
    url: Url,
    headers: HeaderMap,
    body: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    make_request(client.post(url), headers, body).await
}

pub async fn patch(
    url: Url,
    headers: HeaderMap,
    body: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    make_request(client.patch(url), headers, body).await
}

pub async fn put(
    url: Url,
    headers: HeaderMap,
    body: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    make_request(client.put(url), headers, body).await
}

pub async fn delete(
    url: Url,
    headers: HeaderMap,
    body: String,
) -> Result<reqwest::Response, reqwest::Error> {
    let client = reqwest::Client::new();
    make_request(client.delete(url), headers, body).await
}

pub async fn make_request(
    mut method: RequestBuilder,
    headers: HeaderMap,
    body: String,
) -> Result<Response, Error> {
    method = match serde_json::from_str::<HashMap<String, Value>>(body.as_str()) {
        Err(_) => method.body(body),
        Ok(json) => method.json(&json),
    };
    method = method.headers(headers);
    method.send().await
}

pub async fn read_result_text(res: Result<Response, Error>, target_path: String) {
    let text = match res {
        Err(e) => panic!("{:?}", e),
        Ok(r) => r.text().await,
    }
    .expect("Could not read body of response.");

    write_to_file(text, Path::new(&target_path))
        .await
        .expect("Something went wrong writing the result to file!");
}
