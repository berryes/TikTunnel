
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
use serde_json::{json, Value};
use std::iter::Map;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    
    let client:Client = reqwest::Client::new();
    let highway: String = String::from("http://localhost:3000/tiktok");
    let mut headers = HeaderMap::new();
    headers.insert("Cookie",  String::from("asd").parse().unwrap());


    let response:String = client.get(highway)
    .headers(headers)
    .send()
    .await?
    .text()
    .await?;


    println!("{}", response);
    let response_data: Value = serde_json::from_str(&response)?;


    println!("{:?}", response_data["meleg"]);

    Ok(())

}