use std::time::Duration;
use reqwest::{header::{self, HeaderMap}, ClientBuilder, Url};
use crate::{undangle::{Reccomendation, self}, client_manager::{SClient, DC}};


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
}


// getting a recommendation
use serde_json::Value;
pub  async fn recommend( params:String, client:SClient) -> Reccomendation {

    let mut baseURL:Url = Url::parse(&dotenv::var("RECOMMEND_URL").expect("Failed to get RECOMMEND_URL"))
    .expect("failed to parse url");


    /*     baseURL.set_query( Some( format!("{}",params).as_str() ));
 */    


    println!("URL: {}",baseURL);

    // Generating Reqwest client
    let timeout = Duration::new(5,0);
    let reqClient =  ClientBuilder::new().timeout(timeout).build().expect("no client xd");
    

    // headers
    let mut headers = HeaderMap::new();
    headers.insert("User-Agent", client.userAgent.parse().unwrap() );
    headers.insert("Cookie", client.cookie.parse().unwrap() );
    // ^^^ user agent has to be the as the agent which we scraped the cookie with ()


    // Sending request to tiktok servers

    let response: String = reqClient.get(baseURL)
    .headers(headers)
    .send()
    .await.expect("Failed to recive response from tiktok recommendation")
    .text()
    .await.expect("Failed to convert string " /*   status::Custom(Status::InternalServerError,  content::RawJson ( "asd" ) ) */  );

   client.used();

    // converting JSON response
    let data: Value = serde_json::from_str(&response.as_str()).expect("failed to convert to json");

    // Stripping the guts out of the json tree given
    let rec: Reccomendation =  undangle::recommendation(data);


    return rec;




}