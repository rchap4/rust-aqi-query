/*
* Copyright RChapman 2021
* This file is part of rust-aqi-query.
* rust-aqi-query is free software: you can redistribute it and/or modify
* it under the terms of the Affero GNU General Public License as published by
* the Free Software Foundation, either version 3 of the License, or
* (at your option) any later version
* rust-aqi-query is distributed in the hope that it will be useful,
* but WITHOUT ANY WARRANTY; without even the implied warranty of
* MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
* Affero GNU General Public License for more details
* You should have received a copy of the Affero GNU General Public License
* along with rust-aqi-query.  If not, see <https://www.gnu.org/licenses/>.
*/

#![allow(dead_code)]
use chrono::prelude::*;
use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use reqwest::Error;
use serde::Deserialize;
use structopt::StructOpt;

mod prom_support;

//use std::error::Error as AqiError;
use std::fmt;

#[derive(Debug)]
struct AqiError {
    details: String,
}

impl AqiError {
    fn new(msg: &str) -> AqiError {
        AqiError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AqiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl std::error::Error for AqiError {
    fn description(&self) -> &str {
        &self.details
    }
}

#[derive(StructOpt, Debug)]
struct RustAqiQueryCli {
    #[structopt(long = "apikey", env)]
    air_now_api_key: String,

    #[structopt(long = "zipcode", env)]
    zip_code: String,

    #[structopt(long = "prometheus-enabled")]
    prom_enabled: Option<Option<bool>>,
}

#[allow(non_snake_case)]
#[derive(Deserialize, Debug)]
struct AqiData {
    DateObserved: String,
    HourObserved: u32,
    LocalTimeZone: String,
    StateCode: String,
    ReportingArea: String,
    ParameterName: String,
    AQI: u32,
}

#[allow(unused)]
fn get_metric_timestamp(hour: u32) -> i64 {
    let now: DateTime<Local> = Local::now();

    Local
        .ymd(now.year(), now.month(), now.day())
        .and_hms(hour, 0, 0)
        .timestamp_millis()
}

fn get_metric_datetime(
    date: &str,
    hour: u32,
) -> Result<DateTime<Local>, Box<dyn std::error::Error>> {
    let trim_str = date.trim_matches(' ').trim_matches(char::from(0));
    let datepart = NaiveDate::parse_from_str(trim_str, "%Y-%m-%d")?;
    Ok(Local
        .ymd(datepart.year(), datepart.month(), datepart.day())
        .and_hms(hour, 0, 0))
}

fn log_o3_metric(aqi: u32) -> Result<(), AqiError> {
    //prom_support::O3_AQI.metric().set_timestamp_ms(get_metric_timestamp(item.HourObserved));

    prom_support::O3_AQI.set(aqi.into());
    Ok(())
}

fn log_pm10_metric(aqi: u32) -> Result<(), AqiError> {
    prom_support::PM10_AQI.set(aqi.into());
    Ok(())
}

fn log_pm25_metric(aqi: u32) -> Result<(), AqiError> {
    prom_support::PM25_AQI.set(aqi.into());
    Ok(())
}

// fn log_aqi_metric(
//     dt: DateTime<Local>,
//     parameter: &str,
//     aqi: u32,
// ) -> Result<(), AqiError> {
//     prom_support::AQI_GUAGE
//         .with_label_values(&[&dt.to_string(), parameter])
//         .set(aqi.into());
//     Ok(())
// }

async fn get_aqi(request_url: &str, with_prom: bool) -> Result<(), Error> {
    let response = reqwest::get(request_url).await?;

    let aqi_data: Vec<AqiData> = response.json().await?;

    if aqi_data.is_empty() {
        println!("Empty response");
    } else {
        println!(
            "{:>10}{:>10}{:>12}{:>10}",
            "Date", "Hour", "Parameter", "AQI"
        );
        for item in &aqi_data {
            if with_prom {
                match item.ParameterName.as_str() {
                    "O3" => log_o3_metric(item.AQI).unwrap(),
                    "PM10" => log_pm10_metric(item.AQI).unwrap(),
                    _ => (),
                }
            }

            // log_aqi_metric(get_metric_datetime(&item.DateObserved, item.HourObserved).unwrap(),
            //               &item.ParameterName,
            //               item.AQI).unwrap();
            println!(
                "{date:>10}{hour:>width$}{parameter_name:>12}{aqi:>width$}",
                date = item.DateObserved,
                hour = item.HourObserved,
                parameter_name = item.ParameterName,
                aqi = item.AQI,
                width = 10
            );
        }
    }
    Ok(())
}

lazy_static! {
    static ref ARGS: RustAqiQueryCli = RustAqiQueryCli::from_args();
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let zip_code = &ARGS.zip_code;
    let api_key = &ARGS.air_now_api_key;
    let format = String::from("application/json");
    let distance: u32 = 20;

    let request_url: String = format!(
        "http://www.airnowapi.org/aq/observation/zipCode/current/?format={format}&zipCode={zipCode}&distance={distance}&API_KEY={apiKey}",
        format = format,
        zipCode = zip_code,
        distance = distance,
        apiKey = api_key
    );

    let arc_url = std::sync::Arc::new(request_url);
    let is_prom_enabled =
        matches!(ARGS.prom_enabled, Some(None) | Some(Some(true)));

    if is_prom_enabled {
        tokio::spawn(async move {
            let mut collect_interval =
                tokio::time::interval(std::time::Duration::from_secs(3600));
            loop {
                collect_interval.tick().await;
                get_aqi(&arc_url, true)
                    .await
                    .unwrap_or_else(|e| println!("Request error {}", e))
            }
        });
        prom_support::enable_prom().await;
    } else {
        get_aqi(&arc_url, false).await?
    }

    Ok(())
}
