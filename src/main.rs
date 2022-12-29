
/*
Plan 0

- Visiting the website, and accepting the cookie (also saving it)

- making requests to the backend instead from scraping from the HTML!



------
Plan 1
1 Visting the website and scraping from ther
*/

/*
ways to overcome the ip blocking:


- proxies (a lot)
- tor nodes (slow :/)
- vpns

gotta randomize request between these

*/

use reqwest::Client;
use reqwest::header::HeaderMap;
use serde_json::{Value};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
   
    
    let client:Client = reqwest::Client::new();
    let highway: String = String::from("https://www.tiktok.com/api/recommend/item_list/?aid=1988&app_language=en&app_name=tiktok_web&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Win32&browser_version=5.0%20%28Windows%29&channel=tiktok_web&cookie_enabled=true&count=9&device_id=7181251537108059653&device_platform=web_mobile&focus_state=true&from_page=fyp&history_len=2&is_fullscreen=false&is_page_visible=true&os=android&priority_region=&referer=&region=DE&screen_height=883&screen_width=412&tz_name=Europe%2FBudapest&webcast_language=en&msToken=64EFVpph0PFaRtXb68BPf7TiffeAkxUE-6uxgAfQeBBMHeu717tNpRGaMi8J8x2aP91jLzMoKYby3kcLbOTROCAn2ehVWCoSwclPmRhznT6Yr6K9XU-ErWWCfhZ6W6ClSIJM7zK8HJdFtFbbtw==&X-Bogus=DFSzKIVOxczANeMeSkV9EGO8kJ0E&_signature=_02B4Z6wo00001GjktBwAAIDBdKcKo9ZSJwxo9bCAAHmb35");
    let mut headers = HeaderMap::new();
    headers.insert("Cookie",  String::from("asd").parse().unwrap());


    
    let response = client.get(highway)
    .headers(headers)
    .send()
    .await?;
    
    if {
        println!("need to set cookies")
    }
    

    // spent like 2 hours figuring this out btw
    // i mean how to convert a string to json
    println!("{:#?}",);

/* 
    let response_data: Value = serde_json::from_str(&response)?; */


/*     println!("{:?}", response_data["itemList"]); */

    Ok(())

}