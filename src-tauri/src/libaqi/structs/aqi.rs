use serde::{Deserialize, Serialize};
use serde_json::Value;

mod generic_types;

/*
    /pjp-api/v1/rest/aqindex/getIndex/{stationId}

    {
  "@context": {},
  "AqIndex": {
    "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika NO2": "string",
    "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika O3": "string",
    "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika PM10": "string",
    "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika PM2.5": "string",
    "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika SO2": "string",
    "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika st": "string",
    "Data wykonania obliczeń indeksu": "string",
    "Data wykonania obliczeń indeksu dla wskaźnika NO2": "string",
    "Data wykonania obliczeń indeksu dla wskaźnika O3": "string",
    "Data wykonania obliczeń indeksu dla wskaźnika PM10": "string",
    "Data wykonania obliczeń indeksu dla wskaźnika PM2.5": "string",
    "Data wykonania obliczeń indeksu dla wskaźnika SO2": "string",
    "Identyfikator stacji pomiarowej": 0,
    "Kod zanieczyszczenia krytycznego": "OZON",
    "Nazwa kategorii indeksu": "string",
    "Nazwa kategorii indeksu dla wskażnika NO2": "string",
    "Nazwa kategorii indeksu dla wskażnika O3": "string",
    "Nazwa kategorii indeksu dla wskażnika PM10": "string",
    "Nazwa kategorii indeksu dla wskażnika PM2.5": "string",
    "Nazwa kategorii indeksu dla wskażnika SO2": "string",
    "Status indeksu ogólnego dla stacji pomiarowej": true,
    "Wartość indeksu": 0,
    "Wartość indeksu dla wskaźnika NO2": 0,
    "Wartość indeksu dla wskaźnika O3": 0,
    "Wartość indeksu dla wskaźnika PM10": 0,
    "Wartość indeksu dla wskaźnika PM2.5": 0,
    "Wartość indeksu dla wskaźnika SO2": 0
  },
  "links": {
    "additionalProp1": "string",
    "additionalProp2": "string",
    "additionalProp3": "string"
  },
  "meta": {},
  "totalPages": 0
}
*/

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Aqi {
    #[serde(flatten)]
    #[serde(rename = "@context")]
    pub context: Value,
    #[serde(rename = "AqIndex")]
    pub aq_index: AqIndex,
    pub links: Value,
    pub meta: Value,
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AqIndex {
    #[serde(skip)]
    #[serde(
        rename = "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika NO2"
    )]
    pub source_date_index_no2: Option<String>,
    #[serde(
        rename = "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika O3"
    )]
    pub source_date_index_o3: Option<String>,
    #[serde(
        rename = "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika PM10"
    )]
    pub source_date_index_pm10: Option<String>,
    #[serde(
        rename = "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika PM2.5"
    )]
    pub source_date_index_pm2_5: Option<String>,
    #[serde(
        rename = "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika SO2"
    )]
    pub source_date_index_so2: Option<String>,
    #[serde(
        rename = "Data danych źródłowych, z których policzono wartość indeksu dla wskaźnika st"
    )]
    pub source_date_index_st: Option<String>,
    #[serde(rename = "Data wykonania obliczeń indeksu")]
    pub calc_date_index: Option<String>,
    #[serde(rename = "Data wykonania obliczeń indeksu dla wskaźnika NO2")]
    pub calc_date_index_no2: Option<String>,
    #[serde(rename = "Data wykonania obliczeń indeksu dla wskaźnika O3")]
    pub calc_date_index_o3: Option<String>,
    #[serde(rename = "Data wykonania obliczeń indeksu dla wskaźnika PM10")]
    pub calc_date_index_pm10: Option<String>,
    #[serde(rename = "Data wykonania obliczeń indeksu dla wskaźnika PM2.5")]
    pub calc_date_index_pm2_5: Option<String>,
    #[serde(rename = "Data wykonania obliczeń indeksu dla wskaźnika SO2")]
    pub calc_date_index_so2: Option<String>,
    #[serde(rename = "Identyfikator stacji pomiarowej")]
    pub station_id: Option<i64>,
    #[serde(rename = "Kod zanieczyszczenia krytycznego")]
    pub critical_pollution_code: Option<String>,
    #[serde(rename = "Nazwa kategorii indeksu")]
    pub category_index_name: Option<String>,
    #[serde(rename = "Nazwa kategorii indeksu dla wskażnika NO2")]
    pub category_index_name_no2: Option<String>,
    #[serde(rename = "Nazwa kategorii indeksu dla wskażnika O3")]
    pub category_index_name_o3: Option<String>,
    #[serde(rename = "Nazwa kategorii indeksu dla wskażnika PM10")]
    pub category_index_name_pm10: Option<String>,
    #[serde(rename = "Nazwa kategorii indeksu dla wskażnika PM2.5")]
    pub category_index_name_pm2_5: Option<String>,
    #[serde(rename = "Nazwa kategorii indeksu dla wskażnika SO2")]
    pub category_index_name_so2: Option<String>,
    #[serde(rename = "Status indeksu ogólnego dla stacji pomiarowej")]
    pub generic_station_index: bool,
    #[serde(rename = "Wartość indeksu")]
    pub index_value: Option<i64>,
    #[serde(rename = "Wartość indeksu dla wskaźnika NO2")]
    pub index_value_no2: Option<i64>,
    #[serde(rename = "Wartość indeksu dla wskaźnika O3")]
    pub index_value_o3: Option<i64>,
    #[serde(rename = "Wartość indeksu dla wskaźnika PM10")]
    pub index_value_pm10: Option<i64>,
    #[serde(rename = "Wartość indeksu dla wskaźnika PM2.5")]
    pub index_value_pm2_5: Option<i64>,
    #[serde(rename = "Wartość indeksu dla wskaźnika SO2")]
    pub index_value_so2: Option<i64>,
}
