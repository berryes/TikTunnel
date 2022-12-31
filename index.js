const { default: axios } = require("axios");

const express = require('express')
const api = express()


api.get('/discover', async (req, res) => {

  let scraped_response = null
  try{
    scraped_response =  await axios({
      url: "https://www.tiktok.com/api/recommend/item_list/?aid=1988&app_language=en&app_name=tiktok_web&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Win32&browser_version=5.0%20%28Windows%29&channel=tiktok_web&cookie_enabled=true&count=30&device_id=7182687550388520454&device_platform=web_pc&focus_state=true&from_page=fyp&history_len=6&is_fullscreen=false&is_page_visible=true&language=en&os=windows&priority_region=&referer=&region=DE&screen_height=1320&screen_width=3153&tz_name=Europe%2FBudapest&webcast_language=en&msToken=i9KAu_5cP4Ll8L-fOdWUG8jXRMDQq1iJ1a0Chkx1eDFZaK7XfvyWpBsLtnL496s1OwKCEDKl-u35e_t57OA4hRe54ihMaAzUlB0IjxaQ36qaUL9GT82S77_mNZOSH5F0XlbzyzCVQESs1s0=&X-Bogus=DFSzsIVEWQzANcZHSkmr7GO8kJ8-&_signature=_02B4Z6wo00001UDaOKQAAIDAXJmGGYuwytlA2zwAADOd33",
      
      headers:{
        "Accept-Encoding": "*", // this shit was crashing bc i dint include this wtf..
      }
    })

}
catch(error){
  console.log(error)
}
  let recommended = []


  for(let video of scraped_response.data.itemList){
    recommended.push({
      author:{
        nickname: video.author.nickname,
        verified: video.author.verified,
        avatar: video.author.avatarThumb
      },
      video:{
        urls: video.video.bitrateInfo[0].PlayAddr.UrlList,
        cover: video.video.cover,

      }
    })

  }
  
  res.send(recommended)
})

api.listen(1337, () => {
  console.log(`Listening`)
})