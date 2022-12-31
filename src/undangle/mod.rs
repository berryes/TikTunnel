#[derive(Debug)]
struct  Author{
    
    nickname: String,
    
    avatarLarge: String,
    avatarMedium: String,
    avatarThumb: String,

    description: String,
    username: String

}
// Video -> Stats
#[derive(Debug)]
struct Stat{
    comments: i64,
    played: i64,
    shares: i64,
}


// Video -> Stream
#[derive(Debug)]
struct Stream{
    cover: String,
    urls: Vec<String>,
    heigth: i128,
    width: i128,
}

// Reccomendation [2] -> Video
#[derive(Debug)]
struct Video{
    author:Author,
    stats: Stat,
    stream: Stream,
}

#[derive(Debug)]
struct Recommendation{
    videos: Vec<Video>,
}


fn getFuckingString(stringbs:serde_json::Value) -> String{

    let bs:String = stringbs.to_string();

    return bs;
}


pub struct Reccomendation{

}
pub fn recommendation(jData:serde_json::Value)-> Reccomendation {
    
    let rec:Reccomendation = REccomendatio

    println!("touashdiuuhaosd")
    return
}

