use apitester::requests::{delete, patch, post, put, read_result_text};
use apitester::{parse_headers::get_headers, requests::get};
use clap::Parser;
use reqwest::Url;
use std::fs;
use std::io::Error;
use std::path::Path;

#[derive(Parser, Debug)]
struct Cli {
    #[arg(short = 't', long = "type")]
    request_type: String,
    #[arg(short, long)]
    url: Url,
    #[arg(short = 'p', long = "path")]
    file_path: Option<String>,
    #[arg(short = 'n', long = "name")]
    file_name: Option<String>,
    #[arg(short = 'e', long = "extension")]
    file_extension: Option<String>,
    #[arg(short = 'b', long = "body")]
    path_to_body: Option<String>,
}

impl Cli {
    fn get_arg<T>(param: Option<T>, default: T, formatter: Option<fn(v: T) -> T>) -> T {
        match param {
            None => default,
            Some(value) => match formatter {
                None => value,
                Some(formatter) => formatter(value)
            },
        }
    }
}

async fn init() -> Result<(), Error> {
    fs::create_dir_all(Path::new("resources"))?;

    let body = Path::new("resources/body.json");
    match body.exists() {
        false => fs::write(body, "")?,
        _ => ()
    };

    let headers = Path::new("resources/headers.toml");
    match headers.exists() {
        false => fs::write(headers, "")?,
        _ => ()
    };

    fs::create_dir_all(Path::new("output"))?;
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    init().await?;
    let args = Cli::parse();

    let path = Cli::get_arg(args.file_path, String::from("output/"), None);
    let name = Cli::get_arg(args.file_name, String::from("result"), None);
    let extension = Cli::get_arg(args.file_extension, String::new(), Some(|v: String| format!(".{}", v)));
    let target = format!("{}{}{}", &path, &name, &extension);

    let body_path = Cli::get_arg(args.path_to_body, String::from("resources/body.json"), None);
    let body = fs::read_to_string(Path::new(&body_path)).expect("Could not read body file");

    let headers = get_headers();

    match args.request_type.as_str() {
        "GET" => {
            read_result_text(get(args.url, headers, body).await, target).await;
        }
        "POST" => {
            read_result_text(post(args.url, headers, body).await, target).await;
        }
        "PATCH" => {
            read_result_text(patch(args.url, headers, body).await, target).await;
        }
        "PUT" => {
            read_result_text(put(args.url, headers, body).await, target).await;
        }
        "DELETE" => {
            read_result_text(delete(args.url, headers, body).await, target).await;
        }
        _ => println!("Method {} not implemented yet!", args.request_type),
    };

    Ok(())
}
