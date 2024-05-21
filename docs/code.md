## Base URL

`https://api.ennead.cc/starrail/code`

## Description

This API provides information about active and inactive reward codes for the game Star Rail. Each reward code includes details about the code itself and the rewards associated with it.

## Endpoints

Get Reward Codes
Retrieve a list of active and inactive reward codes along with their associated rewards.
- URL: `/starrail/code`
- Method: GET
- Query Parameters:
  - `lang`: (optional) Specifies the language of the response. Supported languages are:
    - `en`: English (default)
    - `cn`: Chinese
    - `tw`: Traditional Chinese
    - `de`: Deutsch
    - `es`: Español
    - `fr`: Français
    - `id`: Indonesian
    - `it`: Italiano
    - `jp`: 日本語
    - `kr`: 한국어
    - `pt`: Português
    - `ru`: Русский
    - `th`: ภาษาไทย
    - `tr`: Türkçe
    - `vn`: Tiếng Việt
- Response Format: application/json

## Response
The response is a JSON object with two main arrays: active and inactive. Each array contains objects representing individual reward codes and their respective rewards.

### Response
<details>
<summary>View Payload Example</summary>

```json
{
  "active": [
    {
      "code": "string",
      "rewards": [
        "string"
      ]
    }
  ],
  "inactive": [
    {
      "code": "string",
      "rewards": [
        "string"
      ]
    }
  ]
}
```
</details>

### Fields

- `active`: An array of objects, each representing an active reward code.
    - `code`: The reward code (string).
    - `rewards`: An array of strings, each representing a reward associated with the code.
- `inactive`: An array of objects, each representing an inactive reward code.
    - `code`: The reward code (string).
    - `rewards`: An array of strings, each representing a reward associated with the code.

## Example Response

English Example
```json
{
  "active": [
    {
      "code": "STARRAILGIFT",
      "rewards": [
        "x50 Stellar Jade",
        "x2 Traveler's Guide",
        "x5 Bottled Soda",
        "x10,000 Credits"
      ]
    },
    {
      "code": "VAJEGY4MNMDK",
      "rewards": [
        "50 Stellar Jades",
        "10k Credits"
      ]
    },
    {
      "code": "ROBININSIDE",
      "rewards": [
        "Adventure Log x2",
        "Credit x10,000"
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
    // More inactive codes...
  ]
}
```

Japanese Example
```json
{
  "active": [
    {
      "code": "STARRAILGIFT",
      "rewards": [
        "明跡釦",
        "漫遊指南",
        "缶入りカコカーラ",
        "信用ポイント"
      ]
    },
    {
      "code": "VAJEGY4MNMDK",
      "rewards": [
        "明跡釦",
        "信用ポイント"
      ]
    }
  ],
  "inactive": [
    {
      "code": "PT8TF72MQ93X",
      "rewards": [
        "明跡釦",
        "信用ポイント"
      ]
    },
    {
      "code": "STPN3TUUTQ8K",
      "rewards": [
        "明跡釦",
        "信用ポイント"
      ]
    },
    {
      "code": "MB6N2TVCSQ9F",
      "rewards": [
        "明跡釦",
        "信用ポイント"
      ]
    }
  ]
}
```

### Usage Example

To retrieve the list of active and inactive reward codes, you would make a `GET` request to the endpoint:

```bash
curl -X GET https://api.ennead.cc/starrail/code

# For codes in Japanese
curl -X GET https://api.ennead.cc/starrail/code?lang=jp
```

The server will respond with a JSON object containing the lists of active and inactive codes along with their associated rewards.

