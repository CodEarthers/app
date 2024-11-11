#![allow(unused)]
#![allow(dead_code)]

use aqi::Aqi;
use data::Sensor2;
use generic_types::DataException;
use reqwest::get;
use sensors::Station2;
use serde::Deserialize;
use serde_json::from_str;
use stations::StationList;
use std::error::Error;
use std::fs::read_to_string;
use std::marker::Sized;

#[path = "./structs/aqi.rs"]
pub mod aqi;

#[path = "./structs/sensors.rs"]
pub mod sensors;

#[path = "./structs/stations.rs"]
pub mod stations;

#[path = "./structs/generic_types.rs"]
pub mod generic_types;

#[path = "./structs/data.rs"]
pub mod data;

static API_URL: &str = "https://api.gios.gov.pl/pjp-api/v1/rest";
static STATIONS: &str = "/station/findAll";
static SENSORS: &str = "/station/sensors/";

static AGGREGATE: &str = "/aggregate/getAggregatePM10Data";

static DATA: &str = "/data/getData/";
static INDEX: &str = "/aqindex/getIndex/";

#[derive(Debug)]
pub enum ApiEnum {
    DataException(DataException),
    Station2(Station2),
    Sensor2(Sensor2),
    Aqi(Aqi),
}
pub trait ApiStruct {}
impl ApiStruct for DataException {}
impl ApiStruct for StationList {}
impl ApiStruct for Station2 {}
impl ApiStruct for Sensor2 {}
impl ApiStruct for Aqi {}

//funkcje
pub async fn find_all() -> StationList {
    let res = include_str!("station_list.json").replace("null", "\"\"");
    let station_list: StationList = test(&res).unwrap();
    station_list
}

pub async fn get_sensors(station_id: i64) -> Result<ApiEnum, Box<dyn Error>> {
    let err: DataException;
    let res = get(format!("{}{}{}", API_URL, SENSORS, station_id))
        .await?
        .text()
        .await?;
    let _x = test(&res);
    match _x {
        Ok(k) => {
            print!("{:#?}", &k);
            return Ok(ApiEnum::Station2(k));
        }
        Err(e) => {
            err = test(&res)?;
            return Ok(ApiEnum::DataException(err));
        }
    }
}

pub async fn get_data(sensor_id: i64) -> Result<ApiEnum, Box<dyn Error>> {
    let err: DataException;
    let res = get(format!("{}{}{}", API_URL, DATA, sensor_id))
        .await?
        .text()
        .await?;
    let _x = test(&res);
    match _x {
        Ok(k) => {
            print!("{:#?}", &k);
            return Ok(ApiEnum::Sensor2(k));
        }
        Err(e) => {
            err = test(&res)?;
            return Ok(ApiEnum::DataException(err));
        }
    }
}

pub async fn get_aqi(station_id: i64) -> Result<ApiEnum, Box<dyn Error>> {
    let err: DataException;
    let res = get(format!("{}{}{}", API_URL, INDEX, station_id))
        .await?
        .text()
        .await?;
    let _x = test(&res);
    match _x {
        Ok(k) => {
            print!("{:#?}", &k);
            return Ok(ApiEnum::Aqi(k));
        }
        Err(e) => {
            err = test(&res)?;
            return Ok(ApiEnum::DataException(err));
        }
    }
}

pub fn test<'a, T: ApiStruct + serde::Deserialize<'a>>(s: &'a str) -> Result<T, serde_json::Error> {
    from_str(s)
}
