
1. Api request comes in (example: https://whatever.media/api/recommend)

2. Check if there is any clients available? (cookies)
    - TRUE | if there isn't one
        - scrape a cookie via a randomized user agent, create a profile for that client in tha database and also set a request counter to 0. after a certain requests which is random the client is dropped/deleted.
        then create the client in the program
    - FALSE | if there is 
        - create the client and set the user agent header

3. Scrape data via the client and modify the usageCount++ 
    - if the usegeCount hits the limit, delete the client

