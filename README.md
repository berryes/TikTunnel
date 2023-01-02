 
<h1 align="center"> TikTunnel </h1>

API for scraping tiktoks.
This program scrapes information from Tiktok and serves an endpoint to safely view the content. 

![Matrix](https://img.shields.io/matrix/tiktunnel?color=%23F39237&server_fqdn=matrix.berryez.xyz&style=for-the-badge)
 ![GitHub issues](https://img.shields.io/github/issues/berryes/tiktunnel?color=%23BF1363&style=for-the-badge)

Yet another project of mine (i start too many new ones and never finish either).


## Todo / plans
Well first of all is actually writing the program. But neverless here are some features I want to implement in the near future. 

- [ ] First, finishing the roadmap 
- [ ] Ai for recommendation
- [ ] Frontend (IOS/Android/Web/Windows/Linux)


## Roadmap 

- [ ] Creating the scraper part.

- [ ] Removing the tiktok watermark
    - There is some way to do it. I'm not aware of such methods rigth now.

- [ ] Building the API via [Rocket](https://rocket.rs)
    - I have only built basic applications with rust so this is going to be a challange for me. I do have experience building fullstack apps with JS tho

- [ ] Overcoming being ip/request banned 
    <br>
    - Such as
        - Randomizing the client infromation if possible (Browser type,Screen size, platform, buildnumber ect)

        - VPNs/Proxies
            via OpenVPN and reqwest proxy.

- Unsure / Migth abandon
    - [ ]   Creating workers
        - Workers are an essential part to a program when it comes to distrubuting the load.
        Many requests at a time can greatly slow the processing speed.



## Why Rust?
First, I was skeptycal about this language. But now after using it for a few projects I have realised its freaking great! Its blazing fast, the memory safety features make sense. So does the syntaxt! I could imagine myself being a Rust developer one day. +1 
