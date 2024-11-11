use lib::ApiEnum;
use serde_json::to_string;

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[path = "./libaqi/lib.rs"]
mod lib;

#[tauri::command]
async fn find_all() -> String {
    let station_list = lib::find_all().await;
    let data = station_list.station_list;
    to_string(&data).unwrap()
}

#[tauri::command]
async fn get_sensors(station_id: i64) -> Result<String, String> {
    let sensors = lib::get_sensors(station_id).await;
    match sensors {
        Ok(k) => match k {
            ApiEnum::Station2(k2) => Ok(to_string(&k2).unwrap()),
            ApiEnum::DataException(e) => Err(to_string(&e).unwrap()),
            _ => Err(String::new()),
        },
        Err(_e) => Err("An error occured".into()),
    }
}

#[tauri::command]
async fn get_data(sensor_id: i64) -> Result<String, String> {
    let data = lib::get_data(sensor_id).await;
    match data {
        Ok(k) => match k {
            ApiEnum::Sensor2(k2) => Ok(to_string(&k2).unwrap()),
            ApiEnum::DataException(e) => Err(to_string(&e).unwrap()),
            _ => Err(String::new()),
        },
        Err(_e) => Err("An error occured".into()),
    }
}

#[tauri::command]
async fn get_aqi(station_id: i64) -> Result<String, String> {
    let aqi = lib::get_aqi(station_id).await;
    match aqi {
        Ok(k) => match k {
            ApiEnum::Aqi(k2) => Ok(to_string(&k2).unwrap()),
            ApiEnum::DataException(e) => Err(to_string(&e).unwrap()),
            _ => Err(String::new()),
        },
        Err(_e) => Err("An error occured".into()),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_geolocation::init())
        .invoke_handler(tauri::generate_handler![
            find_all,
            get_sensors,
            get_data,
            get_aqi
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
