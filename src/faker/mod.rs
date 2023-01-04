// generating random data for basURL params

/* pub struct FakeParams{
    app_language: String, // en hu bg
    app_name: String, // tiktok_web
    browser_language: String, // en hu bg fr
    browser_name: String, // Mozzila, Chrome, Opera, Brave
    browser_online: bool,
    browser_platform: String,  // win32 
    browser_version: String, // 5.0%20%28Windows%29 4.0%23%Android%29
    channel: String, // tiktok_web
    cookie_enabled: bool,
    count: i64, // 1-400
    device_id: i128, // idk whatever
    device_platform: String, // web_mobile web_desktop
    history_len: i64, // 1-99
    is_fullscreen: bool, 
    is_page_visible: bool, // false, true
    os: String, // Androidd, windows, ChromeOS, Linux, Ios
    screen_height: i128, // 1080
    screen_width: i128, // 1920
    tz_name: String, // Europe/Budapest
    webcast_language: String, // en


/*     msToken: String, // no idea what this is yet
    X-Bogus: String, //neither this
    _signature: String, // or this */

    // region: "DE" or whatever else. depends on the request
} */



use rand::{Rng, rngs::ThreadRng};
// give json obejct, get random item from it idk
fn generate_element( avail:serde_json::Value) -> String {
    
    let mut rng:ThreadRng = rand::thread_rng();

    
    let numero: usize = rng.gen_range(0..=avail.as_array().expect("Failed to convert appLanguages/region to object").len() );


    return avail[numero].clone().to_string();
}

use std::fs::File;
use std::io::Read;

// Generates a randoom agent from the random.json file
pub fn agent() -> String{
    // loading data from random.json and pushing it into string
    let mut randoms:String = String::new(); 
    File::open("random.json").unwrap().read_to_string(&mut randoms).expect("failed to read random.json as string");
    
    // stuff from random.json turned into readable format
    let mut json_data: serde_json::Value =  serde_json::from_str( randoms.as_str() ).expect("JSON was not well-formatted");

    let chosen:serde_json::Value = json_data["User-agent"].clone();

    // creating the rng
    let mut rng:ThreadRng = rand::thread_rng();
    let pick: usize = rng.gen_range(0..=chosen.as_array().expect("Failed to convert appLanguages/region to object").len() );

    return chosen[pick].to_owned().to_string();
}


// randomized query generator
pub fn query( mut choso:serde_json::Value ) -> Vec<Vec<String>> {
    
    let mut rng = rand::thread_rng();
    
    let mut params:Vec<Vec<String>> = Vec::new();



    // generating app_language and much more..


    params.push( vec![ 
        String::from("app_language"),
        generate_element( choso["appLanguages/region"].clone() ) 
    ] );
    
    params.push( vec![ 
        String::from("browser_language"), 
        generate_element( choso["appLanguages/region"].clone() ) 
    ] );
    
    params.push( vec![ 
        String::from("browser_name"), 
        generate_element( choso["browsers"].clone() ).to_string() 
    ] );

    // oh yes. nested code
    params.push( vec![ 
        String::from("browser_version"), 
        format!("{} ({})", generate_element( choso["browser_versions"].clone() ).trim(),  generate_element( choso["fullPlatform"].clone() ).trim() ) 
    ] );


    params.push( vec![ 
        String::from("screen_height"), 
        rng.gen_range(400..=3440).to_string() 
    ] );

    params.push( vec![
         String::from("screen_width"), 
         rng.gen_range(400..=3440).to_string() 
    ] );


    params.push( vec![
         String::from("tz_name"), 
         generate_element( choso["tz_names"].clone() ) 
    ] );
    
    params.push( vec![
         String::from("webcast_language"),
          generate_element( choso["appLanguages/region"].clone() ) 
    ] );


    
    return params; 
}

pub fn user_agent(choso:serde_json::Value) -> String {

    return String::from("aoisuhdiaos")

}