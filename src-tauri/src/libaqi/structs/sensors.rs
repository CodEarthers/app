use serde::{Deserialize, Serialize};
use serde_json::Value;

mod generic_types;

// /pjp-api/v1/rest/station/sensors/{stationId}
/*
    {
    "@context": {},
    "Lista stanowisk pomiarowych dla podanej stacji": [
        {
        "Id wskaźnika": 0,
        "Identyfikator stacji": 0,
        "Identyfikator stanowiska": 0,
        "Wskaźnik": "string",
        "Wskaźnik - kod": "string",
        "Wskaźnik - wzór": "string"
        }
    ],
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
pub struct Station2 {
    #[serde(rename = "@context")]
    #[serde(flatten)]
    pub context: Value,
    #[serde(rename = "Lista stanowisk pomiarowych dla podanej stacji")]
    pub sensor_list: Vec<Sensor>,
    pub links: Value,
    pub meta: Value,
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Sensor {
    #[serde(rename = "Id wskaźnika")]
    pub indicator_id: Option<i64>,
    #[serde(rename = "Identyfikator stacji")]
    pub station_id: Option<i64>,
    #[serde(rename = "Identyfikator stanowiska")]
    pub sensor_id: Option<i64>,
    #[serde(rename = "Wskaźnik")]
    pub indicator: Option<String>,
    #[serde(rename = "Wskaźnik - kod")]
    pub indicator_code: Option<String>,
    #[serde(rename = "Wskaźnik - wzór")]
    pub indicator_pattern: Option<String>,
}
