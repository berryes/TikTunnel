
/*
Plan 0

- Visiting the website, and accepting the cookie (also saving it)

- making requests to the backend instead from scraping from the HTML!



------
Plan 1
1 Visting the website and scraping from ther
*/

use reqwest::Client;
use reqwest::header::HeaderMap;



use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct  Stats{
    followerCount: i128,
    heart: i128,
    videoCount: i128,

}
#[derive(Debug, Deserialize)]
struct  Author {
    avatarThumb:String,
    avatarMedium:String,
    avatarLarger:String,
    nickname: String,
}
#[derive(Debug, Deserialize)]
struct  Video {
    author: Author,
    authorStats:Stats,

}
#[derive(Debug, Deserialize)]
struct Reccomendation {
    itemList:Vec<Video>
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    let client:Client = reqwest::Client::new();
    let highway: String = String::from("http://localhost:3000/data");
    let mut headers = HeaderMap::new();
    headers.insert("Cookie",  String::from("asd").parse().unwrap());


    let response: String = client.get(highway)
    .headers(headers)
    .send()
    .await?
    .text()
    .await?;

    println!("{}",&response.as_str());
    let raktestu = serde_json::from_str(response.as_str())?;

    println!("{:#?}", raktestu);


    Ok(())

}