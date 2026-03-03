// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod networking;
mod config;
mod fetching;

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

use tauri::{AppHandle, Emitter};
use crate::fetching::{AppState, add_players, get_players_from_who, init_fetcher};
use crate::networking::request_player;
use crate::config::{init_config_system, read_api_key, write_api_key};

#[tauri::command]
fn req_player(app: AppHandle, name: String) {
    app.emit("request-player", name).unwrap();
}

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
fn remove_player(app: AppHandle, name: String) {
    app.emit("remove-player", name).unwrap();
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
fn initialize_fetcher(app: AppHandle, state: tauri::State<'_, AppState>) {
    println!("[lib::initialize_fetcher] initializing fetcher");
    state.is_running.store(true, Ordering::Relaxed);
    init_fetcher(app, state.interval_ms.clone(), state.is_running.clone());
}

#[tauri::command]
fn stop_fetcher(state: tauri::State<'_, AppState>) {
    state.is_running.store(false, Ordering::Relaxed);
    println!("[lib::stop_fetcher] stopped fetcher");
}

#[tauri::command]
fn update_interval(state: tauri::State<'_, AppState>, newms: u64) {
    state.interval_ms.store(newms, Ordering::Relaxed);
    println!("[lib::update_interval] Polling rate set to {}ms", newms);
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
    init_config_system();

    let interval_ms = Arc::new(AtomicU64::new(1000));
    let is_running = Arc::new(AtomicBool::new(true));

    tauri::Builder::default()
        .manage(AppState {
            is_running,
            interval_ms
        })
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            req_player,
            add_player,
            add_multiple_players,
            remove_player,
            clear_players,
            write_apikey,
            get_apikey,
            initialize_fetcher,
            stop_fetcher,
            update_interval,
            initialize,
            toggle_sidebar,
            toggle_general_settings,
            close_general_settings
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
