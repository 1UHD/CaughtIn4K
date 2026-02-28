// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod networking;
mod config;
mod fetching;

use std::sync::Arc;
use std::sync::atomic::AtomicU64;

use tauri::{AppHandle, Emitter};
use crate::fetching::{add_players, get_players_from_who, init_fetcher};
use crate::networking::request_player;
use crate::config::{init_config_system, read_api_key, write_api_key};

#[tauri::command]
async fn add_player(app: AppHandle, name: String) {
    println!("{}", name);

    request_player(app, name).await;
}

#[tauri::command]
fn add_multiple_players(app: AppHandle, msg: String) {
    println!("[lib::add_mutliple_players] {}", msg);
    let players = get_players_from_who(msg);

    add_players(app, players);
}

#[tauri::command]
fn remove_player(app: AppHandle, uuid: String) {
    app.emit("remove-player", uuid).unwrap();
}

#[tauri::command]
fn clear_players(app: AppHandle) {
    app.emit("clear-players", ()).unwrap();
}

#[tauri::command]
fn write_apikey(apikey: String) {
    match write_api_key(apikey) {
        Ok(_) => {},
        Err(e) => println!("[lib::write_apikey] Couldn't write apikey: {}", e)
    }
}

#[tauri::command]
fn get_apikey(app: AppHandle) {
    match read_api_key() {
        Ok(key) => app.emit("get-apikey", key).unwrap(),
        Err(e) => println!("[lib::get_apikey] Couldn't read apikey: {}", e)
    }
}

#[tauri::command]
fn initialize_fetcher(app: AppHandle) {
    let interval = Arc::new(AtomicU64::new(1000));
    println!("[lib::initialize_fetcher] initializing fetcher");

    init_fetcher(app, interval.clone());
}

#[tauri::command]
fn initialize() {
    init_config_system();
}

#[tauri::command]
fn toggle_sidebar(app: AppHandle) {
    app.emit("toggle-sidebar", ()).unwrap();
}

#[tauri::command]
fn toggle_general_settings(app: AppHandle) {
    app.emit("toggle-general-settings", ()).unwrap();
}

#[tauri::command]
fn close_general_settings(app: AppHandle) {
    app.emit("close-general-settings", ()).unwrap();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            add_player,
            add_multiple_players,
            remove_player,
            clear_players,
            write_apikey,
            get_apikey,
            initialize_fetcher,
            initialize,
            toggle_sidebar,
            toggle_general_settings,
            close_general_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
