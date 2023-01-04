use crate::{DAO, scraper, faker};

pub struct SClient{
   pub userAgent: String,
    pub cookie: String,
    pub usageCount: i128,
    pub usageMax: i128
}
pub struct SError{
    code: i128,
}


pub async fn get() -> Result<SClient,SError>{

    let mut client: SClient;
    let mut clientData = DAO::getRandom(String::from("clients"));
   
    match clientData {
        Ok(map) =>{
            println!("Found a client!");

            // create client from data
            client = SClient{
                userAgent: map.get("userAgent").expect("Failed to convert userAgent").to_owned(),
                cookie: map.get("cookie").expect("Failed to convert cookie").to_owned(),

                usageCount: map.get("usageCount").expect("Failed to convert usagecount").to_owned()
                .parse::<i128>().expect("failed to parse integer from string"),
                
                usageMax: map.get("usageMax").expect("Failed to convert userAgent").to_owned()
                .parse::<i128>().expect("failed to parse integer from string")

            }
        }
        Err(error) =>{
            println!("No client available, geenrating one!");
            // generate client client

            let agent:String = faker::agent();
            let cookies = scraper::cookie(agent.clone()).await;

            println!("cookies given by tiktok: {:?} \n agent used: {}",cookies,agent);
        }
    }

    let client = SClient{
        userAgent: String::from("asdas"),
        cookie: String::from("gecirak"),
        usageCount: 123,
        usageMax: 123123,
    };


    Ok(client)
}

pub async fn generate() -> SClient{

    let client = SClient{
        userAgent: String::from("asdas"),
        cookie: String::from("gecirak"),
        usageCount: 123,
        usageMax: 123123,
    };

    return client;

}