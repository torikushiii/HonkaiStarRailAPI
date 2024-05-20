# Base URL

> `https://api.ennead.cc/starrail/news/events`

> `https://api.ennead.cc/starrail/news/notices`

> `https://api.ennead.cc/starrail/news/info`

### Description

This API provides information about ongoing and upcoming events, info, and notices in the game Star Rail.

### Endpoints
#### Get Events, Info, and Notices

Retrieve a list of current and upcoming events, info, and notices along with their details.

- URL: `/starrail/news/{type}`
    - `{type}` can be one of: `events`, `info`, `notices`
- Method: GET
- Query Parameters:
  - `lang`: (optional) Specifies the languageof the response. Supported languages are:
    - `en`: English (default)
    - `cn`: Chinese
    - `tw`: Traditional Chinese
    - `de`: Deutsch
    - `es`: Español
    - `fr`: Français
    - `id`: Indonesian
    - `it`: Italiano
    - `ja`: 日本語
    - `ko`: 한국어
    - `pt`: Português
    - `ru`: Русский
    - `th`: ภาษาไทย
    - `tr`: Türkçe
    - `vi`: Tiếng Việt
- Response Format: application/json

### Response

The response is a JSON array where each object represents an individual entry with its respective details.

## Example Response for Events
```json
[
  {
    "id": "369",
    "banner": "https://upload-os-bbs.hoyolab.com/upload/2024/05/09/fc465b932b5244ebdbc0c5405c0d7cf9_7867962839364084390.png",
    "createdAt": 1715248802,
    "description": "Welcome to Moonless Midnight: Robin's Rhythmic Festival!\nIn this captivating festival, let's immerse ourselves in the Concerto with Robin~\nWhether you love singing or want to try dancing challenges, you can find your own stage here and have the chance to win exclusive prizes!",
    "endAt": 1720454399,
    "startAt": 1715248800,
    "title": "Events with Rewards: Welcome to Moonless Midnight: Robin's Rhythmic Festival!",
    "url": "https://www.hoyolab.com/article/369"
  }
]
```

#### Japanese Language Example
```json
[
  {
    "id": "369",
    "lang": "ja-jp",
    "banner": "https://upload-os-bbs.hoyolab.com/upload/2024/05/09/1cb3ace503b47756b866fc75ddc9b33f_3097823593824707928.png",
    "createdAt": 1715248802,
    "description": "『月隠りの真夜中：ロビンのリズミックフェスティバル』へようこそ！\n夢中にさせられるフェスティバルで、ロビンと共に協奏状態に入りましょう～\n歌が好きな方でもダンスに挑戦したい方でも、ここで自分だけのステージを見つけ、独占賞品を獲得するチャンスがあります！",
    "endAt": 1720454399,
    "startAt": 1715248800,
    "title": "【コンテスト】『月隠りの真夜中・ロビンのリズミックフェスティバル』へようこそ",
    "url": "https://www.hoyolab.com/article/369"
  }
]
```

## Example Response for Info
```json
[
  {
    "id": "23110430",
    "banner": [
      "https://upload-os-bbs.hoyolab.com/upload/2023/11/20/2121f5c07fccf75dcfe2b3074d76370c_3836051422114685388.jpg",
      "https://upload-os-bbs.hoyolab.com/upload/2023/11/20/637073493248a3d9fec07ddef77d4193_6710269306029640430.jpg"
    ],
    "createdAt": 1700460005,
    "description": "Hello, Trailblazers. Pom-Pom's going to introduce everyone to the Xianzhou Luofu's Fyxestroll Garden this time. Pom-Pom has also specially invited two guests along for the adventure, so you should hu",
    "title": "Travel Encounters (Special Edition)",
    "url": "https://www.hoyolab.com/article/23110430"
  }
]
```

## Example Response for Notices
```json
[
  {
    "id": "22994915",
    "banner": [
      "https://upload-os-bbs.hoyolab.com/upload/2023/11/14/addec2d24a1f50df8d94fec2c70705f1_1634804021748876352.png"
    ],
    "createdAt": 1700020807,
    "description": "The Honkai: Star Rail Project Astro-Warp is about to begin a new round of small-scale full-wipe confidential beta testing. Trailblazers who have successfully signed up before 2023/11/20 09:59 (UTC+8",
    "title": "Project Astro-Warp Beta Sign-Up Reminder",
    "url": "https://www.hoyolab.com/article/22994915"
  }
]
```

### Usage Example

To retrieve the list of current and upcoming events, info, or notices, you would make a `GET` request to the endpoint with the appropriate type:

```bash
# For events
curl -X GET https://api.ennead.cc/starrail/news/events

# For info
curl -X GET https://api.ennead.cc/starrail/news/info

# For notices
curl -X GET https://api.ennead.cc/starrail/news/notices

# For events in Japanese
curl -X GET https://api.ennead.cc/starrail/news/events?lang=ja

# For info in Chinese
curl -X GET https://api.ennead.cc/starrail/news/info?lang=cn
```

The server will respond with a JSON array containing the details as described in the response object section.
