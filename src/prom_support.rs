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

use lazy_static::lazy_static;
use prometheus::{Encoder, IntGauge, Registry};
use warp::Filter;

lazy_static! {
    pub static ref O3_AQI: IntGauge =
        IntGauge::new("airnow_o3_aqi_values", "Airnow O3 AQI")
        .expect("Could not create O3 Metric");

    pub static ref PM10_AQI: IntGauge =
        IntGauge::new("airnow_pm10_aqi_values", "Airnow PM10 AQI values")
        .expect("Could not create PM10 metric");

    pub static ref PM25_AQI: IntGauge =
        IntGauge::new("airnow_pm25_aqi_values", "Airnow PM25 AQI values")
        .expect("Could not create PM2.5 metric");

    // pub static ref AQI_GUAGE: IntGaugeVec =
    //     IntGaugeVec::new(Opts::new("airnow_aqi_values", "Airnow AQI values"), &["observation_time", "parameter"])
    //     .expect("Could not create AQI metric");

    pub static ref REGISTRY: Registry = Registry::new();
}

fn register_metrics() -> Result<(), Box<dyn std::error::Error>> {
    if let Err(e) = REGISTRY.register(Box::new(O3_AQI.clone())) {
        Err(Box::new(e))
    } else {
        Ok(())
    }?;

    if let Err(e) = REGISTRY.register(Box::new(PM10_AQI.clone())) {
        Err(Box::new(e))
    } else {
        Ok(())
    }?;

    if let Err(e) = REGISTRY.register(Box::new(PM25_AQI.clone())) {
        Err(Box::new(e))
    } else {
        Ok(())
    }
    // if let Err(e) = REGISTRY.register(Box::new(AQI_GUAGE.clone())) {
    //     Err(Box::new(e))
    // } else {
    //     Ok(())
    // }
}

async fn metric_handler() -> Result<impl warp::Reply, warp::Rejection> {
    let encoder = prometheus::TextEncoder::new();
    let mut buffer = Vec::new();

    if let Err(e) = encoder.encode(&REGISTRY.gather(), &mut buffer) {
        println!("Couldn't encode metrics: {}", e);
    };

    let mut res = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            println!("Could not convert metrics from utf-8: {}", e);
            String::default()
        }
    };

    buffer.clear();

    let mut buffer = Vec::new();
    if let Err(e) = encoder.encode(&prometheus::gather(), &mut buffer) {
        println!("Could not encode prometheus metrics: {}", e);
    };

    let res_custom = match String::from_utf8(buffer.clone()) {
        Ok(v) => v,
        Err(e) => {
            println!("Custom metrics could not be converted from utf-8: {}", e);
            String::default()
        }
    };

    buffer.clear();
    res.push_str(&res_custom);
    Ok(res)
}

// Change to ...enable_prom() -> Result<...>
pub async fn enable_prom() {
    register_metrics().unwrap();

    let metrics_route = warp::path("metrics").and_then(metric_handler);

    let routes = metrics_route;

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
