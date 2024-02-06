use std::collections::HashMap;
use std::fs::File;
use std::io::copy;
use std::path::PathBuf;
use reqwest::{Error, Response};
use serde_json::json;


#[tokio::main]
async fn main() -> Result<(), Error> {
    let resp = post_url().await.unwrap();
    // println!("{:?}", resp.text().await?);
    let parse_url = parse_url(resp).await.unwrap();
    let _ = download_video(parse_url).await;
    Ok(())
}


async fn post_url() -> Result<Response, Error> {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/121.0.0.0 Safari/537.36".parse().unwrap());
    headers.insert("Cookie", "kpf=PC_WEB; clientid=3; did=web_b6e2297b5940946099a6c9f27c3ac5ba; userId=1722965195; kuaishou.server.web_st=ChZrdWFpc2hvdS5zZXJ2ZXIud2ViLnN0EqABCnchixT_fxx4WAEemI74l85MMffx8ogHK1zxpwWOWB9G-4vKCFgfcedRPLDQgqxbHaoVhssKDmsQjpOakW9T4qDKtQeHAchYCzv6P-8Plm_-d-45BC7X7TMzbonRpblo37gJh8zAsmdyk7dy12xsf6-mC79yxLMp4gfw0OeP8mdy1Jls4ik7Y3WkNFLKHo5_8LZW9yHfk21CSAUcN6SYlxoS9XMBYg26NCtIxdOwhbHEY-u6IiDw_ZSn1a2CT5KHGuR2EXH_Ox8zXBK1iv5y1xRqkfpTxigFMAE; kuaishou.server.web_ph=9e20ae9ffdf4c83874f82617aea448747ac8; kpn=KUAISHOU_VISION".parse().unwrap());
    let data = json!({
        "operationName": "visionProfilePhotoList",
        "variables": {
            "userId": "3x6krpyskdwg4au",
            "pcursor": "",
            "page": "profile"
        },
        "query": "fragment photoContent on PhotoEntity {\n  __typename\n  id\n  duration\n  caption\n  originCaption\n  likeCount\n  viewCount\n  commentCount\n  realLikeCount\n  coverUrl\n  photoUrl\n  photoH265Url\n  manifest\n  manifestH265\n  videoResource\n  coverUrls {\n    url\n    __typename\n  }\n  timestamp\n  expTag\n  animatedCoverUrl\n  distance\n  videoRatio\n  liked\n  stereoType\n  profileUserTopPhoto\n  musicBlocked\n  riskTagContent\n  riskTagUrl\n}\n\nfragment recoPhotoFragment on recoPhotoEntity {\n  __typename\n  id\n  duration\n  caption\n  originCaption\n  likeCount\n  viewCount\n  commentCount\n  realLikeCount\n  coverUrl\n  photoUrl\n  photoH265Url\n  manifest\n  manifestH265\n  videoResource\n  coverUrls {\n    url\n    __typename\n  }\n  timestamp\n  expTag\n  animatedCoverUrl\n  distance\n  videoRatio\n  liked\n  stereoType\n  profileUserTopPhoto\n  musicBlocked\n  riskTagContent\n  riskTagUrl\n}\n\nfragment feedContent on Feed {\n  type\n  author {\n    id\n    name\n    headerUrl\n    following\n    headerUrls {\n      url\n      __typename\n    }\n    __typename\n  }\n  photo {\n    ...photoContent\n    ...recoPhotoFragment\n    __typename\n  }\n  canAddComment\n  llsid\n  status\n  currentPcursor\n  tags {\n    type\n    name\n    __typename\n  }\n  __typename\n}\n\nquery visionProfilePhotoList($pcursor: String, $userId: String, $page: String, $webPageArea: String) {\n  visionProfilePhotoList(pcursor: $pcursor, userId: $userId, page: $page, webPageArea: $webPageArea) {\n    result\n    llsid\n    webPageArea\n    feeds {\n      ...feedContent\n      __typename\n    }\n    hostName\n    pcursor\n    __typename\n  }\n}\n"
    });
    let client = reqwest::Client::new();
    let resp = client.post("https://www.kuaishou.com/graphql")
        .headers(headers)
        .json(&data)
        .send()
        .await?;
    Ok(resp)
}


async fn parse_url(resp: Response) -> Result<HashMap<String, String>, Error> {
    let body: serde_json::Value = resp.json().await?;
    let want = body["data"]["visionProfilePhotoList"]["feeds"].clone();
    let mut data_stream: HashMap<String, String> = HashMap::new();
    for i in want.as_array().unwrap(){
        let name = i["photo"]["caption"].clone();
        let url = i["photo"]["photoUrl"].clone().to_string().replace("\"", "");
        let mut name = name.to_string().replace("\n", "").replace("\"", "").replace("#", "").replace("@", "").replace(" ", "");
        name.push_str(".mp4");
        data_stream.insert(name, url);
    }
    Ok(data_stream)
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