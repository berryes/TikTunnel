

## GET | api/recommend

```
graph TD
    getrecommend(GET api/recommend) --> getClient(Get a client from DB)
    getClient --> noClientInDB(None in DB) --> 
    genRandom(Generate random params and user agent) -->
    scrapeCookie(Scrape cookie via data) --> addDb(Add to db) --> reqWithClient

    getClient --> hasClientInDB(Found one) --> reqWithClient
    
    reqWithClient(Make request with the client) --> OrganizeData(Strip only the useful data) --> CreateproxyLinks(Create proxy links)
    --> buildJSON(Build response JSON) --> respond(Send Response)
```


```
flowchart LR
A[Hard] -->|Text| B(Round)
B --> C{Decision}
C -->|One| D[Result 1]
C -->|Two| E[Result 2]
```