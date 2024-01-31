#[warn(dead_code)]
use std::fs::File;
use serde_json::{Value};
use std::io::{BufReader, copy};
use std::path::PathBuf;
use reqwest::{Error};

async fn download_image(url: &str) -> Result<(), Error> {
    let client = reqwest::Client::new();
    let resp = client.get(url).send().await?;
    let name = url.replace("%", "");
    let mut filename = name.split('=').last().unwrap().to_string();
    filename.push_str(".png");
    let mut dest_path = PathBuf::from("E:\\RustPrograms\\spider\\src\\bin\\images");
    dest_path.push(filename);
    let mut dest = File::create(dest_path.as_path()).unwrap();

    let data = resp.bytes().await?;
    copy(&mut data.as_ref(), &mut dest).unwrap();
    Ok(())
}


async fn download_images(url_list: &[String]) -> Result<(), Error> {
    let str_url_list: Vec<&str> = url_list.iter().map(|s| s.as_str()).collect();
    for url in str_url_list {
        let reurl = url.replace("\"", "");
        download_image(reurl.as_str()).await?;
    }
    Ok(())
}


#[tokio::main]
async fn main() -> Result<(), Error> {
    let file = File::open("E:\\RustPrograms\\spider\\src\\bin\\url.json").unwrap();
    let reader = BufReader::new(file);
    let untyped: Value = serde_json::from_reader(reader).unwrap();

    let want = untyped["app"]["videoDetail"]["images"].clone();

    let mut url_list: Vec<String> = Vec::new();

    for item in want.as_array().unwrap() {
        let url = item["urlList"][0].to_string();
        url_list.push(url);
    }
    download_images(&url_list).await.unwrap();

    // let url_list: Vec<String> = want.as_array().unwrap()
    //     .iter()
    //     .map(|item| item["urlList"][0].to_string()).collect();


    // let url_list: Vec<String> = want.as_array().unwrap()
    //     .iter()
    //     .filter_map(|item|item["urlList"][0].as_str().map(|s| s.to_string()))
    //     .collect();


    Ok(())
}