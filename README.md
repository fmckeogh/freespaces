# [freespaces.work](https://freespaces.work)

[![Deploy](https://github.com/fmckeogh/freespaces/actions/workflows/deploy.yml/badge.svg)](https://github.com/fmckeogh/freespaces/actions/workflows/deploy.yml)

## API

### `GET /locations`

Response:

```json
[
    {
        "name": "Taste",
        "occupancy": "low"
    },
    {
        "name": "Costa",
        "occupancy": "medium"
    },
    {
        "name": "Rector's Cafe",
        "occupancy": "low"
    },
    {
        "name": "Main Library",
        "occupancy": "high"
    }
]
```

### `POST /locations`

Request body:

```json
{
    "name": "Main Library",
    "occupancy": "medium"
}
```
