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



pub struct Reccomendation{
    videos: Vec<Video>,
}

pub fn recommendation(jData:serde_json::Value) {
    
    let rec:Reccomendation;

    let videos:serde_json::Value = jData["itemList"].clone();
    
    for video in videos.as_array().unwrap().iter(){
        
        let author: Author = Author { 
            nickname: video["author"]["nickname"],
            avatarLarge: (),
            avatarMedium: (),
            avatarThumb: (),
            description: (),
            username: ()
        };

        // crerate recomendation
        println!("{:?}",video.get( "author" ))
        

    }


}

