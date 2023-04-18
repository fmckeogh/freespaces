# [freespaces.work](https://freespaces.work)

[![Deploy](https://github.com/fmckeogh/freespaces/actions/workflows/deploy.yml/badge.svg)](https://github.com/fmckeogh/freespaces/actions/workflows/deploy.yml)

## API

### `GET /locations`

Response:

```json
[
	{
		"name": "Costa",
		"occupancy": "high",
		"timestamp": 1681750300
	},
	{
		"name": "Main Library",
		"occupancy": "medium",
		"timestamp": 1681750261
	},
	{
		"name": "Rector's Cafe",
		"occupancy": "low",
		"timestamp": 1681750220
	},
	{
		"name": "Taste",
		"occupancy": "medium",
		"timestamp": 1681750300
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
