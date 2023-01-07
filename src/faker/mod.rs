



use rand::{Rng, rngs::ThreadRng};

// give json obejct, get random item from it idk
fn generate_element( avail:serde_json::Value) -> serde_json::Value {
    let mut rng:ThreadRng = rand::thread_rng();
    let numero: usize = rng.gen_range(0..=avail.as_array().expect("Failed to convert appLanguages/region to object").len() );
    return avail[numero].clone();
}

use std::fs::File;
use std::io::Read;
use std::vec;

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
pub fn query() -> String {
    
/*     aid:1988
    app_language:en
    app_name:tiktok_web
    browser_language:en-US
    browser_name:Mozilla
    browser_online:true
    browser_platform:Win32
    browser_version:5.0%20%28Windows%29
    channel:tiktok_web
    cookie_enabled:true
    count:30
    device_id:7182687550388520454
    device_platform:web_pc
    focus_state:true
    from_page:fyp
    history_len:6
    is_fullscreen:false
    is_page_visible:true
    language:en
    os:windows
    priority_region:
    referer:
    region:DE
    screen_height:1320
    screen_width:3153
    tz_name:Europe%2FBudapest
    webcast_language:en
    msToken:i9KAu_5cP4Ll8L-fOdWUG8jXRMDQq1iJ1a0Chkx1eDFZaK7XfvyWpBsLtnL496s1OwKCEDKl-u35e_t57OA4hRe54ihMaAzUlB0IjxaQ36qaUL9GT82S77_mNZOSH5F0XlbzyzCVQESs1s0=
    X-Bogus:DFSzsIVEWQzANcZHSkmr7GO8kJ8-
    _signature:_02B4Z6wo00001UDaOKQAAIDAXJmGGYuwytlA2zwAADOd33
 */

 
    let mut rng = rand::thread_rng();

    let mut randoms:String = String::new(); 
    File::open("random.json").unwrap().read_to_string(&mut randoms).expect("failed to read random.json as string");
    
    // stuff from random.json turned into readable format
    let mut choso: serde_json::Value =  serde_json::from_str( randoms.as_str() ).expect("JSON was not well-formatted");


     let mut queries:String = String::new(); 

    // what is this monstrosity
     queries.push_str(
        format!("app_language={}&", 
        generate_element( choso["appLanguages/region"].clone() ) )
        .replace('"', "").as_str()
    );
     queries.push_str(
        format!("browser_language={}&", 
        generate_element( choso["appLanguages/region"].clone() ) )
        .replace('"', "").as_str()
    );
     queries.push_str(
        format!("browser_name={}&", 
        generate_element( choso["browsers"].clone() ) )
        .replace('"', "").as_str()
    );
     queries.push_str(
        format!("browser_version={} ({})&",generate_element( choso["browser_versions"].clone()  ),
        generate_element( choso["fullPlatform"].clone()  ) )
        .replace('"', "").as_str()
    );

     queries.push_str(
        format!("screen_height={}&", 
        rng.gen_range(400..=3440).to_string()  )
        .replace('"', "").as_str()
    );
     queries.push_str(
        format!("screen_width={}&", 
        rng.gen_range(400..=3440).to_string() )
        .replace('"', "").as_str()
    );
     queries.push_str(
        format!("tz_name={}&", 
        generate_element( choso["tz_names"].clone() ) )
        .replace('"', "").as_str()
    );
     queries.push_str(
        format!("webcast_language={}&", 
        generate_element( choso["appLanguages/region"].clone() ) )
        .replace('"', "").as_str()
    );

    
    queries.push_str("aid=1988&
    browser_online=true&
    channel=tiktok_web&
    device_platform=web_pc&
    device_id=7182687550388520454&
    msToken=i9KAu_5cP4Ll8L-fOdWUG8jXRMDQq1iJ1a0Chkx1eDFZaK7XfvyWpBsLtnL496s1OwKCEDKl-u35e_t57OA4hRe54ihMaAzUlB0IjxaQ36qaUL9GT82S77_mNZOSH5F0XlbzyzCVQESs1s0=&
    X-Bogus=DFSzsIVEWQzANcZHSkmr7GO8kJ8-&
    _signature=_02B4Z6wo00001UDaOKQAAIDAXJmGGYuwytlA2zwAADOd33
    ");

    println!("{}",queries);

    return queries; 
}

pub fn user_agent(choso:serde_json::Value) -> String {

    return String::from("aoisuhdiaos")

}