 
<h1 align="center"> TikTunnel </h1>

An API for scraping tiktoks written in Rust!

This program scrapes data from Tiktok and serves an endpoint to safely view the content. 


 ![GitHub issues](https://img.shields.io/github/issues/berryes/tiktunnel?color=%23BF1363&style=for-the-badge) ![GitHub last commit](https://img.shields.io/github/last-commit/berryes/tiktunnel?color=%23BF1363&logoColor=%23BF1363&style=for-the-badge) ![GitHub](https://img.shields.io/github/license/berryes/tiktunnel?color=%23BF1363&logoColor=%23BF1363&style=for-the-badge)


## Todo / plans
Well first of all is actually writing the program. But neverless here are some features I want to implement in the near future. 

- [ ]  Finishing the roadmap 
- [ ] Ai for recommendation?
- [ ] Frontend (IOS/Android/Web/Windows/Linux)?


## Roadmap 

- [ ] Creating the scraper part.

- [ ] Removing the tiktok watermark
    - There is some way to do it. I'm not aware of such methods rigth now.

- [ ] Building the API via [Rocket](https://rocket.rs)
    - I have only built basic applications with rust so this is going to be a challange for me. I do have experience building fullstack apps with JS tho

- [ ] Overcoming being ip/request banned 
    <br>
    - With 
        - Randomizing the client infromation if possible (Browser type,Screen size, platform, buildnumber ect)

        - VPNs/Proxies
            via OpenVPN and reqwest proxy.


- Unsure / Migth abandon
    - [ ]   Creating workers
        - Workers are an essential part to a program when it comes to distrubuting the load.
        Many requests at a time can greatly slow the processing speed.



