# Rust AQI Query

## Overview
Between traffic smog and wildfire smoke it's useful to know the current air quality.  If your a runner or a cyclist planning your day, you might monitor the air quality along with the weather.  This tool queries the AirNow API and provides the current air quality index without leaving your shell.  

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

Zip Code and API key can be provided as a command line options or set as environment variables.  The Rust StructOpt crate facilities this behavior.  See source code for details.

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
        --prometheus-enabled <prom-enabled>    
        --zipcode <zip-code>                    [env: ZIP_CODE=]
```

## TO DOs
* Setup Github Action (on Linux) for Clippy, Build and Format
* Refactor functions to modules
* Metrics are provided to prometheus in a basic way, think harder on the ideal data model
* Include zipcode in metrics
* Improve error handling
* Add trace support
* Parameters for bind IP and port
* Documentation for running on Docker

