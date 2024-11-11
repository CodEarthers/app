#![allow(unused)]

use std::collections::HashMap;
use std::fs;
use std::io::ErrorKind;

use generic_types::DataException;
use serde_json::from_str;
use libaqi::*;
use stations::StationList;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let station_list: StationList = find_all().await;
    /*
    let station2 = get_sensors(17880).await?;

    match &station2 {
        ApiEnum::Station2(k) => print!("{:#?}", k),
        ApiEnum::DataException(e) => print!("{:#?}", e),
        _ => ()
    }

    let data = get_data(29680).await?;

    match &data {
        ApiEnum::Sensor2(k) => print!("{:#?}", k),
        ApiEnum::DataException(e) => print!("{:#?}", e),
        _ => ()
    }
    */
    let aqi = get_aqi(17880).await?;

    match &aqi {
        ApiEnum::Aqi(k) => print!("{:#?}", aqi),
        ApiEnum::DataException(e) => print!("{:#?}", e),
        _ => ()
    }

    Ok(())
}
