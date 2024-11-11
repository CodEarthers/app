use serde::{Deserialize, Serialize};
use serde_json::Value;

mod generic_types;

// pjp-api/v1/rest/station/findAll
/*
    {
    "@context": {},
    "Lista stacji pomiarowych": [
        {
        "Gmina": "string",
        "Identyfikator miasta": 0,
        "Identyfikator stacji": 0,
        "Kod stacji": "string",
        "Nazwa miasta": "string",
        "Nazwa stacji": "string",
        "Powiat": "string",
        "Ulica": "string",
        "WGS84 λ E": "string",
        "WGS84 φ N": "string",
        "Województwo": "string"
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
#[serde[rename_all = "camelCase"]]
pub struct StationList {
    #[serde(rename = "@context")]
    pub context: Value,
    #[serde(rename = "Lista stacji pomiarowych")]
    pub station_list: Vec<Station>,
    pub links: Value,
    pub meta: Value,
    pub total_pages: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde[rename_all = "PascalCase"]]
pub struct Station {
    pub gmina: String,
    #[serde(rename = "Identyfikator miasta")]
    pub city_id: i64,
    #[serde(rename = "Identyfikator stacji")]
    pub station_id: i64,
    #[serde(rename = "Kod stacji")]
    pub station_code: String,
    #[serde(rename = "Nazwa miasta")]
    pub city_name: String,
    #[serde(rename = "Nazwa stacji")]
    pub station_name: String,
    pub powiat: String,
    pub ulica: String,
    #[serde(rename = "WGS84 λ E")]
    pub wsg84_lambda_e: String,
    #[serde(rename = "WGS84 φ N")]
    pub wsg84_phi_n: String,
    #[serde(rename = "Województwo")]
    pub wojewodztwo: String,
}
