use serde::{Deserialize, Serialize};
use serde_json::Value;

mod generic_types;

/*
    /pjp-api/v1/rest/data/getData/{idSensor}

    {
  "@context": {},
  "Lista danych pomiarowych": [
    {
      "Data": "string",
      "Kod stanowiska": "string",
      "Wartość": 0
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
pub struct Sensor2 {
    #[serde(rename = "@context")]
    #[serde(flatten)]
    pub context: Value,
    #[serde(rename = "Lista danych pomiarowych")]
    pub data_list: Vec<Data>,
    pub links: Value,
    pub meta: Value,
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "Data")]
    pub date: Option<String>,
    #[serde(rename = "Kod stanowiska")]
    pub sensor_code: Option<String>,
    #[serde(rename = "Wartość")]
    pub value: Option<f64>,
}
