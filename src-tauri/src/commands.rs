use core::panic;
use std::collections::HashMap;
use reqwest::Url;
use tauri_app::{
    requests::{get, post, put, patch, delete, read_result_text},
    utils::headermap_from_hashmap,
};

#[tauri::command]
pub async fn do_request(request_type: &str, url: Url, headers: &str, body: String) -> Result<String, ()> {
    let h_map: HashMap<&str, &str> = serde_json::from_str(headers).expect("Headers are not valid json");
    let header_map = headermap_from_hashmap(h_map.iter());

    let res = match request_type {
        "GET" => get(url, header_map, body).await,
        "POST" => post(url, header_map, body).await,
        "PUT" => put(url, header_map, body).await,
        "PATCH" => patch(url, header_map, body).await,
        "DELETE" => delete(url, header_map, body).await,
        _ => panic!("No method like that defined!")
    };

    Ok(read_result_text(res).await)
}
