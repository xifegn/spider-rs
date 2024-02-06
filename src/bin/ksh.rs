use std::collections::HashMap;
use std::fs::File;
use rand::{thread_rng, Rng};
use std::io::copy;
use std::path::PathBuf;
use rand::distributions::Alphanumeric;
use reqwest::{Error, Response};
use serde_json::{json};
use async_recursion::async_recursion;
use async_once_cell::OnceCell;
use lazy_static::lazy_static;
use async_std::sync::Mutex;


lazy_static! {
    static ref DONE: Mutex<bool> = Mutex::new(false);
}


static USERID: &str = "3xhtw3p338e7ayu";
static PCURSOR: &str = "1.692324812239E12";
static ONCE: OnceCell<()> = OnceCell::new();

#[tokio::main]
async fn main() -> Result<(), Error> {
    let _ = scheduler(USERID, "").await.unwrap();
    Ok(())
}


async fn post_url(pcursor: &str, user_id: &str) -> Result<Response, Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Cookie", "kpf=PC_WEB; clientid=3; did=web_b6e2297b5940946099a6c9f27c3ac5ba; userId=1722965195; kuaishou.server.web_st=ChZrdWFpc2hvdS5zZXJ2ZXIud2ViLnN0EqABhSAQdHJ-UuJFr33hSsWdq5UfQt8vhKdppxIun7L-J5ECoW4XWaEUTqA-aba8AjFbqqVmiBxKV0gdgGSA2pjBsco3Qlk6gUGdvmcIKlE8TDqSSCy7zfdHYD_rcdvVYga76qGVbB3EWkzTgc9IzIdMfnE8WALerGIj0bEHCjsCNHbhKkMPkAE_Sgkr7rnnZqcMFYyi64-lPvu8svNu-7IaSxoSQ7mEvdT3xtmzMcd5yI94lmh4IiDrf0v3ZPdQGtvI1IM18aPj8iKZuiPRJKrQgBIG5YJCESgFMAE; kuaishou.server.web_ph=db77ce7d5547f0061595bc8ab009ea6b2468; kpn=KUAISHOU_VISION".parse().unwrap());
    let mut data = HashMap::new();
    let mut variables = HashMap::new();
    variables.insert("userId".to_string(), user_id.to_string());
    variables.insert("pcursor".to_string(), pcursor.to_string());
    variables.insert("page".to_string(), "profile".to_string());
    data.insert("operationName".to_string(), "visionProfilePhotoList".to_string());
    let variable = json!(variables).to_string();
    data.insert("variables".to_string(), variable);
    data.insert("query".to_string(), "fragment photoContent on PhotoEntity {\n  __typename\n  id\n  duration\n  caption\n  originCaption\n  likeCount\n  viewCount\n  commentCount\n  realLikeCount\n  coverUrl\n  photoUrl\n  photoH265Url\n  manifest\n  manifestH265\n  videoResource\n  coverUrls {\n    url\n    __typename\n  }\n  timestamp\n  expTag\n  animatedCoverUrl\n  distance\n  videoRatio\n  liked\n  stereoType\n  profileUserTopPhoto\n  musicBlocked\n  riskTagContent\n  riskTagUrl\n}\n\nfragment recoPhotoFragment on recoPhotoEntity {\n  __typename\n  id\n  duration\n  caption\n  originCaption\n  likeCount\n  viewCount\n  commentCount\n  realLikeCount\n  coverUrl\n  photoUrl\n  photoH265Url\n  manifest\n  manifestH265\n  videoResource\n  coverUrls {\n    url\n    __typename\n  }\n  timestamp\n  expTag\n  animatedCoverUrl\n  distance\n  videoRatio\n  liked\n  stereoType\n  profileUserTopPhoto\n  musicBlocked\n  riskTagContent\n  riskTagUrl\n}\n\nfragment feedContent on Feed {\n  type\n  author {\n    id\n    name\n    headerUrl\n    following\n    headerUrls {\n      url\n      __typename\n    }\n    __typename\n  }\n  photo {\n    ...photoContent\n    ...recoPhotoFragment\n    __typename\n  }\n  canAddComment\n  llsid\n  status\n  currentPcursor\n  tags {\n    type\n    name\n    __typename\n  }\n  __typename\n}\n\nquery visionProfilePhotoList($pcursor: String, $userId: String, $page: String, $webPageArea: String) {\n  visionProfilePhotoList(pcursor: $pcursor, userId: $userId, page: $page, webPageArea: $webPageArea) {\n    result\n    llsid\n    webPageArea\n    feeds {\n      ...feedContent\n      __typename\n    }\n    hostName\n    pcursor\n    __typename\n  }\n}\n".to_string());
    let client = reqwest::Client::new();
    let resp = client.post("https://www.kuaishou.com/graphql")
        .headers(headers)
        .json(&data)
        .send()
        .await?;

    Ok(resp)
}


#[async_recursion]
async fn parse_url(user_id: &str, resp: Response) -> Result<Option<HashMap<String, String>>, Error> {
    let body: serde_json::Value = resp.json().await?;
    let want = body["data"]["visionProfilePhotoList"]["feeds"].clone();
    // println!("{:?}", want);
    let mut data_stream: HashMap<String, String> = HashMap::new();
    for i in want.as_array().unwrap(){
        let name = i["photo"]["caption"].clone();
        let url = i["photo"]["photoUrl"].clone().to_string().replace("\"", "");
        let mut name = name.to_string().chars().filter(|c| !"#@\n\" ".contains(*c)).collect::<String>();
        name.push_str(".mp4");
        if data_stream.contains_key(&name) {
            let rng = thread_rng();
            let suffix: String = rng
                .sample_iter(&Alphanumeric)
                .take(6)
                .map(char::from)
                .collect();
            let mut name = name.replace(".mp4", "");
            name.push_str(&suffix);
            name.push_str(".mp4");
            data_stream.insert(name, url);
        }else {
            data_stream.insert(name, url);
        }
    }
    let pcursor = body["data"]["visionProfilePhotoList"]["pcursor"].clone();
    // println!("{:?}", pcursor);
    return if pcursor.as_str().unwrap() != "no_more" {
        scheduler(user_id, pcursor.as_str().unwrap()).await.unwrap();
        Ok(Some(data_stream))
    } else {
        let mut done = DONE.lock().await;
        if *done {
            Ok(None)
        }else {
            ONCE.get_or_init(async {
                let _ = post_url(PCURSOR, user_id).await.unwrap();
            }).await;
            *done = true;
            Ok(Some(data_stream))
        }
    }
}


async fn scheduler(user_id: &str, pcursor: &str) -> Result<(), Error> {
    let resp = post_url(pcursor, user_id).await.unwrap();
    // println!("{:?}", resp.text().await);
    let stream = parse_url(user_id, resp).await.unwrap();
    // println!("{:?}", stream.unwrap());
    if stream != None {
        let _ = download_video(stream.unwrap()).await;
    }
    Ok(())
}


async fn download_video(data: HashMap<String, String>) -> Result<(), Error> {
    let path = PathBuf::from("E:\\RustPrograms\\spider\\src\\bin\\videos");
    for (filename, url) in data.iter() {
        let mut path = path.clone();
        path.push(filename);
        let mut file = File::create(path.as_path()).unwrap();
        let client = reqwest::Client::new();
        let resp = client.get(url.as_str()).send().await?;
        let data = resp.bytes().await?;
        copy(&mut data.as_ref(), &mut file).unwrap();
    }
    Ok(())
}