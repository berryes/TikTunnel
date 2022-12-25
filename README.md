# TikTunnel
 API for scraping tiktoks. Esentially a proxy
This program scrapes information from Tiktok and serves an endpoint to safely view content. 


Yet another project of mine (i start too many new ones and never finish either).

Due to TikToks invasive behaviour when it comes to privacy, I have fleed from the platform long ago. I my self have realised that I was addicted to the short dopamine rushes and the content on Tiktok. I don't wanna miss out on anything on there and after all I was happy using it. So I decided to start a harder project in Rust. 

This project is basically scraping all the content from the website and serving it to the end user to safely view it. Of course I have bigger plans (like implementing an AI based recommendation system ) but for now thats the main goal.



## Todo / plans
Well first of all is actually writing the program. But neverless here are some features I want to implement in the near future. 

[ ] Finishing roadmap
[ ] Ai for recommendation
[ ] Frontend (IOS/Android/Web/Windows/Linux)


## Roadmap 

- [ ] Finding a way to scrape data
    - Tiktok is smart. They (such as other websites ) have implemented Anti-Scraping techniques. They use randomized classes, ids for  their elements. Example: tiktok-yf3ohr-DivContainer e1yey0rl0
    It makes scraping harder in many ways, but after finding connections between elements you can figure out what is supossed to be what. 

- [ ] Removing the tiktok watermark
    - There is some way to do it. I'm not aware of such methods rigth now.

- [ ] Building the API via [Rocket](https://rocket.rs)
    - I have only built basic applications with rust so this is going to be a challange for me. I do have experience building fullstack apps with JS tho

- [ ] Privacy features
    - I want to implement other privacy features to make it impossible for tiktok to assosiate any data. 

    - Such as
        - Randomizing the client infromation if possible (Browser type,Screen size, platform, buildnumber ect)
        - VPN tunnels. I want to implement this with OpenVPN. The user adds the .ovpn files/urls to the program  and it randomly switches between them when scraping ( if possible )


## Why Rust?
First, I was skeptycal about this language. But now after using it for a few projects I have realised its freaking great! Its blazing fast, the memory safety features make sense. So does the syntaxt! I could imagine myself being a Rust developer one day. +1 

Yes there migth be easier languages to write this program in. 
I know its a bit overkill. But I myself try to keep code as efficent and fast as possible no matter what!
