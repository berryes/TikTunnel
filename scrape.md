

1, We visit the [website](https://tiktok.com) and scrape the cookie 
    - We will use the cookie to request data straigth from the backend skipping the painful method of crawling in the html tree and extracting from there.

2, Getting reccomended via the cookie (https://www.tiktok.com/api/recommend/item_list/)

3, Snipping the data so only necesarry information (likes,cover,music,video, uploader ect) is left.
    - To reduce data transerred.







Routes

Recommended
https://www.tiktok.com/api/recommend/item_list/
    - Required Params:
        - aid: 1988
        - count: 9

    - Could be useful
        - region: DE,HU ect..
        - os: android 
