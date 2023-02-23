 
<p align="center">
<img  src="https://github.com/berryes/TikTunnel/blob/main/logo.png?raw=true"  width="160">
</p>

<h1 align="center"> TikTunnel 

<br>


 ![GitHub issues](https://img.shields.io/github/issues/berryes/tiktunnel?color=%23BF1363&style=for-the-badge) ![GitHub last commit](https://img.shields.io/github/last-commit/berryes/tiktunnel?color=%23BF1363&logoColor=%23BF1363&style=for-the-badge) ![GitHub](https://img.shields.io/github/license/berryes/tiktunnel?color=%23BF1363&logoColor=%23BF1363&style=for-the-badge)

</h1>


An API for scraping tiktoks.

This program scrapes data from TikTok and serves an endpoint to safely view the content. 

### Current status: <p style="color:purple"> Reverse engineering API... </p>
Note: I lost motivation for researching tiktoks api and because of that the project has been put to a hold.


## Todo / plans
Well first of all is actually writing the program. But neverless here are some features I want to implement in the near future. 

- [ ]  Finishing the roadmap 
- [ ] Frontend (IOS/Android/Web/Windows/Linux)?


## Roadmap 

- [ ] Figuring out how to scrape data.

- [ ] Creating the scraper.

- [ ] Removing the tiktok watermark
    - There is some way to do it. I'm not aware of such methods rigth now.

- [ ] Building the API via [Rocket](https://rocket.rs)

- [ ] Overcoming being ip/request banned 
    <br>
    - With 
        - Randomizing the client information(Browser type,Screen size, platform, buildnumber user agent, ect)
        - Proxies/VPNs
            via OpenVPN and reqwest proxy.

- [ ]   Creating workers
       - Workers are an essential part to a program when it comes to distrubuting the load. Many requests at a time can greatly slow the processing speed.

# Contributing
You can always make a pull request if you have a fix for a possible bug or idea.
If you want to help me reverse engineer the API you can always reach me at 
[#tiktunnel:berryez.xyz](https://matrix.to/#/#tiktunnel:berryez.xyz)
