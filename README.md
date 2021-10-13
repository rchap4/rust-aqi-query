# Rust AQI Query

## Overview
Between traffic smog and wildfire smoke it's useful to know the current air quality.  If your a runner or a cyclist planning your day, you might monitor the air quality along with the weather.  This tool queries the AirNow API and provides the current air quality index without leaving your shell.  

Optionally, this cool can poll the AirNow API hourly and expose the data to Prometheus at /metrics.

Uses the AirNow API to pull air quality data for a US zip code, where the data is available.  You'll need to obtain a free API key to use this tool.    

## Air Now API Info
[https://docs.airnowapi.org/](https://docs.airnowapi.org/)

## Build

```
cargo build
# or
cargo build --release
```

## Usage Info

Zip Code and API key can be provided as a command line options or set as environment variables.  The Rust StructOpt crate facilities this behavior.  See source code or CLI help for details.

With *--prometheus-enabled true* exposes a endpoint at /metrics for Prometheus to scrape.

```
rust-aqi-query 0.2.0

USAGE:
    rust-aqi-query [OPTIONS] --apikey <air-now-api-key> --zipcode <zip-code>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --apikey <air-now-api-key>              [env: AIR_NOW_API_KEY=]
        --ip <bind-ip>
        --port <port>
        --prometheus-enabled <prom-enabled>
        --zipcode <zip-code>                    [env: ZIP_CODE=]
```

## Run on Docker

Build container using rust:latest, debian:buster-slim, Prometheus support enabled.
```
    # Build container - same directory as Dockerfile
    docker build -t rust-aqi-query .
```

Setup .env file

```
ZIP_CODE=123456
AIR_NOW_API_KEY=YOUR-AIR-NOW-API-KEY

```

Run container with Docker using default bridge network.  

```
    # Assuming image built above tagged as rust-aqi-query:lastest

    docker run \
    --env-file .env \
    --publish 3030:3030 \
    --rm \
    rust-aqi-query:latest

```

## TO DOs
* Setup Github Action (on Linux) for Clippy, Format, and Build
* Refactor functions to modules
* Metrics are provided to prometheus in a basic way, think harder on the ideal data model
* Include zipcode in metrics
* Improve error handling
* Add trace support
* Parameters for bind IP and port
* Documentation for running on Docker
* Mask API key enviroment variable and enable Prometheus related options as environment variables

