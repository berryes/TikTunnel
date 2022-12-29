const { default: axios } = require("axios");


async function  main(){

    let client = axios.create({headers:{}})
    let response = await axios({
        method: 'get',
        url: 'https://www.tiktok.com/api/recommend/item_list/?aid=1988&app_language=en&app_name=tiktok_web&browser_language=en-US&browser_name=Mozilla&browser_online=true&browser_platform=Win32&browser_version=5.0%20%28Windows%29&channel=tiktok_web&cookie_enabled=true&device_id=7162895929715656197&focus_state=true&from_page=fyp&history_len=6&is_fullscreen=false&is_page_visible=true&priority_region=&referer=&region=DE&webcast_language=en&msToken=EL8UPntphpXh-_GjuWZTm70vlBJQT-vyc61f7fsgH0ZxPnfxcklJ4LQlQj_eg9Exwujjdo1VGk5jJ2MHWRzUACILO5l5FrmacuY-usGPYx1oL8Mmppue0abKAdROhdqyD-R401cTafw_Ts4k&X-Bogus=DFSzsIVuBHhANaFQSkjT6TO8kJMJ&_signature=_02B4Z6wo0000199WhGwAAIDCwxU60brK8U.fV4DAAJSGd7&count=9&device_id=7181251537108059653&device_platform=web_mobile&history_len=3&os=android&screen_height=883&screen_width=412&tz_name=Europe%2FBudapest&msToken=kJKjaipYpv6DBtbefhJnuJEKyShLVz_Q2jugzt9Ol8JHCwT32tApVilq4x9vxWbLkbQpmrnJtUzGz_al5CCKh1S4TbY_78bbbVFE-o6GikCgjQ3UqoA2dk4rvWcB_q4kuV5nkDInTQMyfHAD&X-Bogus=DFSzKIVOhiUANeMeSkV2UGO8kJM4&_signature=_02B4Z6wo00001sbByrwAAIDD2oJ0A-8DPgrG0M4AANInf7&history_len=2&msToken=64EFVpph0PFaRtXb68BPf7TiffeAkxUE-6uxgAfQeBBMHeu717tNpRGaMi8J8x2aP91jLzMoKYby3kcLbOTROCAn2ehVWCoSwclPmRhznT6Yr6K9XU-ErWWCfhZ6W6ClSIJM7zK8HJdFtFbbtw==&X-Bogus=DFSzKIVOxczANeMeSkV9EGO8kJ0E&_signature=_02B4Z6wo00001GjktBwAAIDBdKcKo9ZSJwxo9bCAAHmb35',
      });

      if(response.headers.has("set-cookie")){
        
        let cookies = response.headers["set-cookie"]
        let headers = {}
        
        for(let cookie of cookies ){
            headers[cookie.slice(0,cookie.indexOf("="))] = cookie.slice(cookie.indexOf("=")+1,cookie.length)
        }
        
        client = axios.create( { headers:headers } )
        
        let data = await  client({
            method: 'get',
            url: 'https://www.tiktok.com/api/recommend/item_list/?aid=1988&count=9',
        })
        console.log(data.data.)
      }

    
}

main()