# Stations Microservice

## Development

* Run `docker-compose up` to start the dependent services
* Install `diesel_cli` with https://diesel.rs/guides/getting-started.html#installing-diesel-cli
* Run `diesel migration run` to apply schema locally


## API

### GET /stations

Returns a list of all stations.

### GET /stations/{id}/status

Returns the status of a station.

### POST /stations

```json
{
  "url": "https://example.com/stations.json"
}
```


Enqueues a job to import the stations from the given URL.