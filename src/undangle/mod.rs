use rocket::serde::{Deserialize,Serialize};

// this below me allows to to magik
#[derive(Serialize, Deserialize, Debug)]
pub struct  Author{
    
    nickname: String,
    
    avatarLarge: String,
    avatarMedium: String,
    avatarSmall: String,

    description: String,
    username: String

}
// Video -> Stats
#[derive(Serialize, Deserialize, Debug)]
pub struct Stat{
    comments: i64,
    played: i64,
    shares: i64,
}


// Video -> Stream
#[derive(Serialize, Deserialize, Debug)]
pub struct Stream{
    cover: String,
    urls: Vec<String>,
    heigth: i128,
    width: i128,
}

// Reccomendation [2] -> Video
#[derive(Serialize, Deserialize, Debug)]
pub struct Video{
    author:Author,
    stats: Stat,
    stream: Stream,
}


#[derive(Serialize, Deserialize, Debug)]
pub struct Reccomendation{
    videos: Vec<Video>,
    response_time: i128,
}

pub fn recommendation(jData:serde_json::Value) -> Reccomendation {
    
    let videos:Vec<Video> = Vec::new();

    let mut rec:Reccomendation = Reccomendation{
        videos: videos,
        response_time: 0,
    };

    let videos:serde_json::Value = jData["itemList"].clone();
    
    for video in videos.as_array().unwrap().iter(){
        let authorData = &video["author"];

        // extracting values from json and creating author object
        let author: Author = Author { 
            nickname: authorData["nickname"].to_string(),
            avatarLarge: authorData["avatarLarger"].to_string(),
            avatarMedium: authorData["avatarMedium"].to_string(),
            avatarSmall:  authorData["avatarThumb"].to_string(),
            description: authorData["signature"].to_string(),
            username: authorData["uniqueId"].to_string()
        };


        // parsing out the stream urls via this painful looking method
        let mut urls: Vec<String> = Vec::new();
        for link in video["video"]["bitrateInfo"][0]["PlayAddr"]["UrlList"].as_array().unwrap().iter(){
            urls.push(link.to_string())
        };

        // creating stream object
        let stream: Stream = Stream { 
            cover: video["video"]["cover"].to_string(),
            urls: urls,
            heigth: 100,
            width: 200,
        };

        // creating Stats object
        let stats: Stat = Stat { 
            comments: video["stats"]["commentCount"].as_i64().expect("Failed to convert stats/commentCount to i64"),
            played: video["stats"]["playCount"].as_i64().expect("Failed to convert stats/playCount to i64"),
            shares: video["stats"]["shareCount"].as_i64().expect("Failed to convert stats/shareCount to i64")
         };

        
         // Assembling parts into a video
        let outter: Video = Video{
            author: author, 
            stream: stream,
            stats: stats,
        };
        
        // adding it into the recommendation object
        rec.videos.push(outter);
    } // end of video assembly

    return rec;

}

