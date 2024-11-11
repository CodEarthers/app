use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct Context {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Links {}

#[derive(Serialize, Deserialize, Debug)]
pub struct Meta {}

/*
    {
    "error_code": "string",
    "error_help": "string",
    "error_reason": "string",
    "error_result": "string",
    "error_solution": "string"
    }
*/

#[derive(Serialize, Deserialize, Debug)]
pub struct DataException {
    pub error_code: String,
    pub error_help: String,
    pub error_reason: String,
    pub error_result: String,
    pub error_solution: String,
}
