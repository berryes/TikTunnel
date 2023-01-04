use std::time::Duration;
use reqwest::{header::{self, HeaderMap}, ClientBuilder};
use crate::{undangle::{Reccomendation, self}, client_manager::SClient};


pub fn search( params:Vec<String>, search_query:String, cookie:String )  {


}


// getting a cookie which is used to scrape data
pub async fn cookie(agent:String) -> Result<Vec<String>,String> {
 
    
    // Generating Reqwest client
    let timeout = Duration::new(5,0);
    let client =  ClientBuilder::new().timeout(timeout).build().expect("Failed to create client");
    
    let response = client.get("https://www.tiktok.com/api")
    .header("User-Agent",agent.as_str())
    .send()
    .await
    .expect("Failed to recive a response from tiktok");
    

    let mut cookies:Vec<String> = Vec::new();

    for cookie in response.headers().get("set-cookie").iter(){

            // defining a tempString cus i dont knwo what 
        // the trait `Sized` is not implemented for `str`rust
        // supossed to mean. pls just ignore this
        let mut tempCook: String = String::new();
        tempCook.push_str(cookie.to_str().expect("Failed to convert cookie into STR"));
        cookies.push(tempCook);  
    }

    Ok(cookies)
//                      vec[0] = user-agent 
//                      vec[1] = cookie
}


// getting a recommendation
use serde_json::Value;
pub  async fn recommend( params:Vec<String>, client:SClient) -> Reccomendation {

    let baseURL = String::from("https://www.tiktok.com/api/recommend/item_list/?aid=1988&app_language=en&app_name=tiktok_web&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Win32&browser_version=5.0%20%28Windows%29&channel=tiktok_web&cookie_enabled=true&count=30&device_id=7182687550388520454&device_platform=web_pc&focus_state=true&from_page=fyp&history_len=6&is_fullscreen=false&is_page_visible=true&language=en&os=windows&priority_region=&referer=&region=DE&screen_height=1320&screen_width=3153&tz_name=Europe%2FBudapest&webcast_language=en&msToken=i9KAu_5cP4Ll8L-fOdWUG8jXRMDQq1iJ1a0Chkx1eDFZaK7XfvyWpBsLtnL496s1OwKCEDKl-u35e_t57OA4hRe54ihMaAzUlB0IjxaQ36qaUL9GT82S77_mNZOSH5F0XlbzyzCVQESs1s0=&X-Bogus=DFSzsIVEWQzANcZHSkmr7GO8kJ8-&_signature=_02B4Z6wo00001UDaOKQAAIDAXJmGGYuwytlA2zwAADOd33");


    
    // Generating Reqwest client
    let timeout = Duration::new(5,0);
    let reqClient =  ClientBuilder::new().timeout(timeout).build().expect("no client xd");
    

    // headers
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", client.userAgent.parse().unwrap() );
    // ^^^ user agent has to be the as the agent which we scraped the cookie with ()


    // Sending request to tiktok servers
    let response: String = reqClient.get(baseURL)
    .headers(headers)
    .send()
    .await.expect("Failed to recive response from tiktok recommendation")
    .text()
    .await.expect("Failed to convert string " /*   status::Custom(Status::InternalServerError,  content::RawJson ( "asd" ) ) */  );

    // converting JSON response
    let data: Value = serde_json::from_str(&response.as_str()).expect("failed to convert to json");

    // Stripping the guts out of the json tree given
    let rec: Reccomendation =  undangle::recommendation(data);


    return rec;




}