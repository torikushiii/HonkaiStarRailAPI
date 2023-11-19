# Honkai: Star Rail API

## *This API is still under development*

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
