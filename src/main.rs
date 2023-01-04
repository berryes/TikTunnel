
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
use reqwest::{ Url};
use serde_json;
use std::{fs::File, io::Read};
use undangle::Reccomendation;
use rocket::tokio::time::{sleep, Duration};
use reqwest::header::HeaderMap;
use rocket::serde::json::Json;

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

    let paramus:String =  faker::query();
    
    // scraping from tiktok.com/api/recommend.....
    let rec:Reccomendation = scraper::recommend(paramus, client)
    .await;

    // replying to the request with the proxied data
    Json(rec)

}


extern crate dotenv;
use dotenv::dotenv;
use std::env;

use std::path::Path;
// building the rocket project
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    
    dotenv().ok();

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

