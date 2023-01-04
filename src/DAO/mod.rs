use std::collections::HashMap;
use sqlite::Connection;


pub fn get(table:String,param:String){

}


pub fn getRandom(table:String) -> Result<HashMap<String,String>,bool>{
    
    // creating connection
    let connection: Connection = sqlite::open("database.sqlite").unwrap();

    let query:String = format!("
    SELECT * FROM {}
    ORDER BY RANDOM()
    LIMIT 1
    ",table);
    
    let mut data:HashMap<String,String> = HashMap::new();

    connection
    .iterate(query, |pairs| {
        for &(name, value) in pairs.iter() {

            data.insert(name.to_string(), value.unwrap().to_string());
        }
        true
    })
    .unwrap();

    // in case nothing in db
    if data.len() == 0{
        return Err(false)
    }

    Ok(data)
}


pub fn initDB(){

    let connection: Connection = sqlite::open("database.sqlite").unwrap();
    
    connection.execute("
    CREATE TABLE IF NOT EXISTS clients (cookie TEXT, usageCount INTEGER,usageMax INTEGER, userAgent TEXT);
    CREATE TABLE IF NOT EXISTS usedClients (cookie TEXT, usageCount INTEGER, userAgent TEXT);


    ")
    .unwrap();

}
