use rand::rngs::ThreadRng;
use sqlite::Connection;

use crate::{DAO, scraper, faker};

/* classDiagram
    class Client {
      id String
      cookie String
      usageCount i128
      usageMax i128
      userAgent String 
     - available()
     - used()
    } */

    
// same layout as in DB
pub struct SClient{
    pub id: String, // uuid
    pub cookie: String,
    pub usageCount: i128,
    pub usageMax: i128,
    pub userAgent: String
}


pub trait usage {
     fn available(&self) -> bool; // returns if you can use this client
     fn used(&self)-> bool; // returns increases client usageCount in db by +1
}

impl usage for SClient{

     fn available(&self) -> bool{

        let connection: Connection = sqlite::open("database.sqlite").unwrap();
        let mut usageCount:i128 = 0;

        connection
        .iterate(format!("SELECT usageCount,usageMax FROM clients WHERE id = '{}' ",&self.id), |pairs| {
            for &(name, value) in pairs.iter() {
                if name == "usageCount" {
                    usageCount = value.unwrap().parse::<i128>().expect("Faiiled to parse usageCount");
                }else{
                    if usageCount+1 > value.unwrap().parse::<i128>().expect("failed to parse"){
                        return false
                    }

                }
            }
            true
        })
        .unwrap();

        return true
    }
     fn used(&self) -> bool{

        return false
    }
}


// GETTING /GENERATING CLIENTS
use uuid::Uuid;
use rand::Rng;

pub async fn get() -> SClient{

    let mut client: SClient;

    // getting a random client from the database
    let mut clientData = DAO::getRandom(String::from("clients"));
   
    // handeling errors
    match clientData {
        // if a client was found
        Ok(map) =>{
            println!("Found a client!");

            // create client from data
            client = SClient{
                id: map.get("id").expect("Failed to get client ID from map").to_owned(),

                userAgent: map.get("userAgent").expect("Failed to convert userAgent").to_owned(),
                cookie: map.get("cookie").expect("Failed to convert cookie").to_owned(),

                usageCount: map.get("usageCount").expect("Failed to convert usagecount").to_owned()
                .parse::<i128>().expect("failed to parse integer from string"),
                
                usageMax: map.get("usageMax").expect("Failed to convert userAgent").to_owned()
                .parse::<i128>().expect("failed to parse integer from string")

            };

           /*  let avail = client.available(); */

        } // ok


        // If no client was found
        Err(error) =>{
            println!("No client available, geenrating one!");
            // generate client client

            // generating AGENT from random.json
            let agent:String = faker::agent();


            // scaraping cookies from tiktok via the agent
            let mut cookies: Vec<String> = Vec::new();
            let ExpectedCookies = scraper::cookie(agent.clone()).await;
            match ExpectedCookies {
                Ok(baked_cookies) =>{
                    cookies = baked_cookies;
                }
                Err(err) =>{
                    println!("Failed to scrape cookies: {}",err);
                }
            }

            // maximum usage getting generated based on the CLIENT_MAX_USE env 
            let mut rng:ThreadRng = rand::thread_rng();
            let maxUse: usize = rng.gen_range(0..=dotenv::var("CLIENT_MAX_USE").unwrap().parse::<usize>().expect("failed to parse CLIENT_MAX_USE"));
            let id = Uuid::new_v4();

            println!("{}",id.to_string());
            // creating SClient object
            client = SClient{
                id: id.to_string(),
                userAgent: agent.clone(),
                cookie: cookies[0].clone(),
                usageCount: 0,
                usageMax: i128::try_from(maxUse).unwrap(),
            };


            // Creating database connection -> creating client entry/row   
            let connection: Connection = sqlite::open("database.sqlite").unwrap();
            let query:String = format!( "INSERT INTO clients VALUES ('{}','{}',{},{},'{}')",
            client.id,
            client.cookie,
            client.usageCount,
            client.usageMax,
            client.userAgent);

            connection.execute(query).expect("Failed to add client to db");

/*             println!("cookies given by tiktok: {:?} \n agent used: {}",cookies,agent);
 */        }
    }

    // returning finished client
    return client;
}

