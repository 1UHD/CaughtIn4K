// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod networking;
mod config;

use networking::{request_player};
use crate::config::{init_config_system, read_api_key, write_api_key};

#[tauri::command]
async fn add_player(name: String) {
    println!("{}", name);
    init_config_system();
    match write_api_key(name) {
        Ok(_) => {},
        Err(e) => println!("couldnt write: {}", e)
    }

    match read_api_key() {
        Ok(apikey) => println!("APIKEY: {}", apikey),
        Err(e) => println!("couldnt read: {}", e)
    }
    //request_player(name).await;
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_player])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
