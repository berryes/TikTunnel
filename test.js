let cookies = [
    'msToken=tcqq4ZNL8-32Wkjk5IpBHbmz9OwHU3hKl09EV9e1EcWEGahdCUad6CylwdGMnr36Bx9D2D29Zve8vdd7hIkqAGUopQqVTKiBlqhm8GiqB02wOwQ0mh_xXjw=; expires=Sun, 08 Jan 2023 02:49:26 GMT; domain=tiktok.com; path=/; secure; SameSite=None',
    '_abck=2A9F1189B301166BD0FEF9BEAAB507E4~-1~YAAQEO8QAgQQhVqFAQAAdNPIWwmkaTaIpXiXmYzL3c7oEg4ZJHxmuRvnaDihRSN7B1WSOojiCFhsCiYDkNBZKA6XiGizP+Z3XZzBIYPXCkyzojWaLruAUhKYaIZ3uZSTTkLjerb16l5pay4TJZrxF8yP0RjYBXnhvhkw9orgx3ACT955MrFDK5h3msTj87vlAQpwpvZoddm02XPhCg/1TgjWBYsbaIU1U/cYdjVg4Xcwz001WFyQv/UC9KyOnnYqM+6MD1TxrUCtowX45JN+x4DdbYZNiHCxmeFh2nO5thDGbEh0x3OMzODvF6wx2AOAEzdsJvJS0ATD3YkQ2om+toh0+V1+zueYrRM9krkJq07hkYRiqlP9ePc=~-1~-1~-1; Domain=.tiktok.com; Path=/; Expires=Fri, 29 Dec 2023 02:49:26 GMT; Max-Age=31536000; Secure',
    'bm_sz=814E051B636E4E7B8ACDEAA5297C06BD~YAAQEO8QAgUQhVqFAQAAdNPIWxKo2JcI19pB+1Y2Xca0HT7Wxrd0CsPcicM82oYPx/5cUiSASa0YmfFw4KbUwO3a9Cxwqxoe2sQtoIjFw9ZJbEmvRaLeo9HugYrCc+DrHqHpBMAj1p9aiYg2IdziPWpB47K+Kbv4dBH99PwYzklJR2mu5AX6Bl5sWEsEkLp3tBYDOi6HALofTqGK5t/CMKIiirkxAQKCze+/hE1iVHGLh4ta71YPBo4XnHFXez1mOsQ7MkM8mQaYNPkHuZ5ry23Y1WuM//GvyZHx1yxChxcCKDQ=~3291188~3687220; Domain=.tiktok.com; Path=/; Expires=Thu, 29 Dec 2022 06:49:26 GMT; Max-Age=14400'
]


let client ={
    headers: {}
} 

for(let cookie of cookies ){

    client.headers[cookie.slice(0,cookie.indexOf("="))] = cookie.slice(cookie.indexOf("=")+1,cookie.length)
}

console.log(client)