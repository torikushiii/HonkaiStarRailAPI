# Honkai: Star Rail API

**Hosted API at https://api.ennead.cc/starrail**

### API Endpoints

#### `/code`

Returns a list of all currently active and inactive redemption codes.

> [GET] https://api.ennead.cc/starrail/code

> Returns: Redemption Code Object
<details>
<summary>View Payload Example</summary>

```json
{
  "active": [
    {
      "code": "STARRAILGIFT",
      "rewards": [
        "50 Stellar Jades",
        "EXP materials"
      ]
    },
    {
      "code": "5S9BND25CRBK",
      "rewards": [
        "Stellar Jade x50",
        "Credits x10000"
      ]
    },
    {
      "code": "SOULWLT4EB",
      "rewards": [
        "Random stuff"
      ]
    }
  ],
  "inactive": [
    {
      "code": "PT8TF72MQ93X",
      "rewards": [
        "Stellar Jade x50",
        "Credits x10000"
      ]
    },
    {
      "code": "STPN3TUUTQ8K",
      "rewards": [
        "Stellar Jade x50",
        "Credits x10000"
      ]
    }
}
```
</details>

### `/news`

Returns a list of all news posts from HoyoLab.

> [GET] https://api.ennead.cc/starrail/news/events

> [GET] https://api.ennead.cc/starrail/news/notices

> [GET] https://api.ennead.cc/starrail/news/info

> Returns: News Object

<details>
<summary>View Payload Example</summary>

```json
[
  {
    "id": "23026259",
    "banner": "https://upload-os-bbs.hoyolab.com/upload/2023/11/16/e27793e2003beb11916a5bd9eacdec98_4928498229263854311.png",
    "createdAt": 1700133925,
    "description": "Every time she gets scared, the fearful Huohuo likes to make paper dolls to bolster her courage Though this time, she hopes that you can help her with the doll-making process! Share the dolls you ma",
    "endAt": 1700495999,
    "startAt": 1700128800,
    "title": "Trailblazer, Can You Help Huohuo Draw a Paper Doll...?",
    "url": "https://www.hoyolab.com/article/23026259"
  },
  {
    "id": "321",
    "banner": "https://upload-os-bbs.hoyolab.com/upload/2023/11/15/d2e99b01ce19ca9f72a00d6de2efaab6_8736161163324272808.png",
    "createdAt": 1700109003,
    "description": "Dear Trailblazers, Honkai: Star Rail version 1.5 \"The Crepuscule Zone\" is now live, and our strategy guide collection contest for version 1.5 will also officially begin today~\nKeen to share your game experience with everyone? Go to HoYoLAB and participate in the competition for a chance to win prizes such as merchandise and Stellar Jades!",
    "endAt": 1705075199,
    "startAt": 1700109000,
    "title": "Prize Event — Version 1.5 \"The Crepuscule Zone\" Strategy Guide Contest Event Begins!",
    "url": "https://www.hoyolab.com/article/321"
  }
]
```
