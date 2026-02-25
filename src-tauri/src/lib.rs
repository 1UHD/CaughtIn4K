// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod networking;
mod config;

use tauri::{AppHandle, Emitter};
use crate::networking::{Player};
use crate::config::{init_config_system, read_api_key, write_api_key};

async fn request_player(app: AppHandle, name: String) {
    let mut player = Player {
        uuid: None, name: name, rank: None, staffrank: None,
        monthlyrank: None, rankcolor: None, bedwars_level: None,
        final_kills: None, fkdr: None, final_deaths: None,
        wins: None, losses: None, wlr: None,
    };

    let uuid = player.get_uuid().await;
    if uuid.is_none() {
        println!("nicked player found");
        app.emit("add-player", player).unwrap();
        return;
    }
    player.uuid = uuid;

    let apikey = match read_api_key() {
        Ok(key) => key,
        Err(e) => {
            println!("error with api key{}", e);
            return;
        },
    };

    player.get_hypixel_player(apikey).await;
    println!("{:?}", player);
    
    app.emit("add-player", player).unwrap();
}

#[tauri::command]
async fn add_player(app: AppHandle, name: String) {
    println!("{}", name);
    
    request_player(app, name).await;
}

#[tauri::command]
fn write_apikey(apikey: String) {
    match write_api_key(apikey) {
        Ok(_) => {},
        Err(e) => println!("[lib::write_apikey] Couldn't write apikey: {}", e)
    }
}

#[tauri::command]
fn initialize() {
    init_config_system();
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_http::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![add_player, write_apikey, initialize])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
