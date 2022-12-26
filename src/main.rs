use scraper::{Html, Selector};


/*
Plan 0

- Visiting the website, and accepting the cookie (also saving it)

- making requests to the backend instead from scraping from the HTML!



------
Plan 1
1 Visting the website and scraping from ther
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get("https://www.tiktok.com/api/recommend/item_list/?aid=1988&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Win32&channel=tiktok_web&cookie_enabled=true&count=30&device_platform=web_pc&focus_state=true&from_page=fyp&is_fullscreen=false&is_page_visible=true&language=en&os=windows&region=DE&screen_height=1200&screen_width=2867&tz_name=Europe%2Moscow&webcast_language=en")
        .await?;

        // parsing videos from html

    println!("{:?}",response);

    Ok(())
}