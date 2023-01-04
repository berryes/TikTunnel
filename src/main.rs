
/*
Plan 0

- Visiting the website, and accepting the cookie (also saving it)

- making requests to the backend instead from scraping from the HTML!

- 

ways to overcome the ip/request  blocking:
- proxies (a lot)
- tor nodes (slow :/)
- vpns

- randomizing header & queryy/param data.
    like:
        - User-Agent
        - Cookie
        - count
        - device_platform
        - channel
        - language
        - tz_name
        - region


gotta randomize request between these

*/


/*
proxy stuff:
since it wouldn't be anonymous if the we sent the user the raw links of 
the response from the tiktok api

like: https://p77-sign-va-lite.tiktokcdn.com/obj/tos-maliva-p-0068/842232d3e72a4f0c8ec72388e627b02c_1669583400?x-expires=1672466400&x-signature=mfoG5%2BlIZsbwbOoixNPKrl3FnKU%3D

we gotta create a rust proxy for it that actually requests it.
sad

https://rocket.rs 
*/


// imports
use rand::random;
use reqwest::{Client, ClientBuilder, Url};
use rocket::response::{status, content};
use serde_json::{Value};
use std::{fs::File, io::Read};
use undangle::Reccomendation;
use rocket::tokio::time::{sleep, Duration};
use reqwest::header::HeaderMap;
use rocket::serde::json::Json;
use faker::user_agent;
// modules
mod undangle;
mod faker;
mod scraper;
mod client_manager;
mod DAO;

// whetever this shit does
#[macro_use] extern crate rocket;

// Api route of recommend
#[get("/api/recommend")]
async fn recommend() -> Json<Reccomendation> {

    let mut client = client_manager::get()
    .await;




    let highway: String = String::from("https://www.tiktok.com/api/recommend/item_list/?aid=1988&app_language=en&app_name=tiktok_web&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Win32&browser_version=5.0%20%28Windows%29&channel=tiktok_web&cookie_enabled=true&count=9&device_id=7181251537108059653&device_platform=web_mobile&focus_state=true&from_page=fyp&history_len=2&is_fullscreen=false&is_page_visible=true&os=android&priority_region=&referer=&region=DE&screen_height=883&screen_width=412&tz_name=Europe%2FBudapest&webcast_language=en&msToken=64EFVpph0PFaRtXb68BPf7TiffeAkxUE-6uxgAfQeBBMHeu717tNpRGaMi8J8x2aP91jLzMoKYby3kcLbOTROCAn2ehVWCoSwclPmRhznT6Yr6K9XU-ErWWCfhZ6W6ClSIJM7zK8HJdFtFbbtw==&X-Bogus=DFSzKIVOxczANeMeSkV9EGO8kJ0E&_signature=_02B4Z6wo00001GjktBwAAIDBdKcKo9ZSJwxo9bCAAHmb35");
   

    // headers
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", String::from("PostmanRuntime/7.30.0").parse().unwrap() );
    // ^^^ user agent has to be the as the agent which we scraped the cookie with ()


    let mut baseURL = Url::parse("https://www.tiktok.com/api/recommend/item_list/?aid=1988&device_id=7181251537108059653&count=9").expect("failed to parse url");


    // Generating randomized url-params/queries
    let mut randoms:String = String::new(); 
    File::open("random.json").unwrap().read_to_string(&mut randoms).expect("failed to read random.json as string");
    let mut generator: serde_json::Value =  serde_json::from_str( randoms.as_str() ).expect("JSON was not well-formatted");
    let params =  faker::query(generator);
    let mut queryStack: String = String::new(); // all the params get pushed in one string | this one
    
    // TODO: remove "" from string cus for some retarded reason it includes it. idk bro
    for param in params {
        queryStack.push_str( format!("{}={}&", param[0].trim(),param[1].trim() ).as_str() );
    }; 
    


    let paramus =  vec![String::from("gyaszrak")];

    let rec = scraper::recommend(paramus, String::from("_abck=59173084B4A16A79E453203E7AB3F693~-1~YAAQFe8QAv6y1FqFAQAA+4M8eQni7RpVsZwHoFOIGTaTD9aEc8UrYKq8rN+P6lsZBYfVk8HJiBev/fYj7bYzeYBWhQY7WP/oOZXd7jYoQw8eQHmVqujkoIpe+MLpGc4ga/k0mgX2p6T8TIm+9VLQkaADW+f5apC7CbqWyKIn/gjvKMnKIN2UuyNoAcj66hfSYg05JwyXWwGiJGq6Bn0lyZrJNwuCGttVg7BmXXlqoMzQtqFMGAweUkucRSduQ8RzhxEQadJFmc6jtHi/futrjmibZ8s43Dk2jse+cBj2BfyRTSBFegh4Oykjpd3lOxXKap1q0iOq98AQunIg/d31WPepqEwriwBj2X7tpsbkWrc3bC0bIqZaY1MWAXkmj63IM3xAHryZDZ+o3eAq4lsMVB2sagXXUcyO~0~-1~-1; ak_bmsc=9036A6C931D8D6EEEEF361DE68F5E9F4~000000000000000000000000000000~YAAQEO8QAptLFnmFAQAAu5xGeRIYaIsD4bZTgQXnr5PcQCxWszAAPgW/04y/aiqsmlcigTIDwN9qoYpeNhsS537U0kMSynWN+FTtQsNv2IJYpjnlBano01V3461nTPWtjoFSkoNFsZTbqxrRxq7GjHF40K+oEeB8VCNwYgv0pt8lENKKwgHlmGDXfyuAerfM4rIz1OqHs6pp9DEOu7scfLt5PY7M1f94AqgbJgt6/Uzk1H1W8Uj9HoS2hLAVpUAA3SW4s13mGkK6mBeYbIyx/Qhm9PADgU3zYOH3rWcOZ2gOjJo4FZtrxHfj+gPvY1UTq5NT+10iYpU4IJbP8abGmn+Ff5L0AuaKLoXxFR2hMSbv0UudUUVc7muoCZQ=; bm_sz=8644D592BDB95F2C2C49B04D6F2EC43D~YAAQEO8QApxLFnmFAQAAu5xGeRJ6sfxPh8QikayDLp1cKY75qZL4/TrFEff0krLFputQ/xahUarVF87hBF7p45eSfUlfbqlq0m+wA3Vo7zVZFfenfMglFqrsHzDelDt06ltK9kLtyCCe8PArgNPd7VEfYi/KOXpA2wBy+OutY/faSlXNU+YkOijBCL6rTRqU7tVi8SagdYDijOquL/KVksl0RpgCc6+Dp47bXXIHFqtqpFAmphLCjh6Ibl8QBEkKORKd8mcMRpeRyPyeWKnICVb7ZKpPE/MAmLNa4dRppDoxbU8=~3621176~3617078; msToken=EuiB9hvFGULih_17WYO4sKu94C37SI8LSDYhmS6KBs8grkYZBdNMtiAbBmS3_D7A7CFsaGlJXyU34KOiQUaPDpjHOB6MsDE5us_RgHg7xNqRUz8MZ3YWRHgfhPuUHXfhBgRCQ7foNxxVzGM="), headers)
    .await;

    // replying to the request with the proxied data
    Json(rec)

}



use std::path::Path;
// building the rocket project
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    
    // Creating database file if it doesn't exists
    if !Path::new("database.sqlite").exists() { 
        File::create("database.sqlite").expect("DB already exists");
    };
    DAO::initDB();
    


    // random.js validation & checking
    let _rocket = rocket::build()
        .mount("/", routes![recommend])
        .launch()
        .await?;

    Ok(())
}

