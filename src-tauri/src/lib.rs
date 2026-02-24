// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod networking;
mod config;

use crate::networking::{Player};
use crate::config::{init_config_system, read_api_key, write_api_key};

async fn request_player(name: String) {
    let mut player = Player {
        uuid: None, name: name, rank: None, staffrank: None,
        monthlyrank: None, rankcolor: None, bedwars_level: None,
        final_kills: None, fkdr: None, final_deaths: None,
        wins: None, losses: None, wlr: None,
    };

    let uuid = player.get_uuid().await;
    if uuid.is_none() {
        println!("nicked player found");
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
}

#[tauri::command]
async fn add_player(name: String) {
    println!("{}", name);
    
    request_player(name).await;
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
