use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader, Seek, SeekFrom};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};
use regex::Regex;
use tauri::AppHandle;
use tauri::async_runtime::spawn;
use tokio::time::{sleep, Duration};
use crate::networking::request_player;

pub fn add_players(app: AppHandle, players: Vec<String>) {
    for player in players {
        let app_clone = app.clone();

        spawn(async move {
            println!("[fetching::add_players] starting call for {}", player);

            request_player(app_clone, player).await;
        });
    }
}

fn get_log_path() -> Option<PathBuf> {
    //'/Users/xxx/Library/Application Support/PrismLauncher/instances/1.8.9/minecraft/logs/latest.log'
    return env::var_os("HOME").map(|home| {
        let mut path = PathBuf::from(home);
        path.push("Library");
        path.push("Application Support");
        path.push("PrismLauncher");
        path.push("instances");
        path.push("1.8.9");
        path.push("minecraft");
        path.push("logs");
        path.push("latest.log");

        println!("{:?}", path);
        return path;
    });
}

pub fn get_players_from_who(who_message: String) -> Vec<String> {
    // [14:08:14] [Client thread/INFO]: [CHAT] ONLINE: C0le_20_Palmer, Kane_2007, CrystalCool11, ...
    let players = who_message.split(": ").collect::<Vec<&str>>()[2];

    let players_str = players.split(", ").collect::<Vec<&str>>();
    return players_str.into_iter().map(|p| p.to_string()).collect();
}

pub fn change_interval(interval_ms: &Arc<AtomicU64>, new_ms: u64) {
    interval_ms.store(new_ms, Ordering::Relaxed);
}

pub fn init_fetcher(app: AppHandle, interval_ms: Arc<AtomicU64>) {
    if let Some(log_path) = get_log_path() {
        start_fetcher(app, interval_ms, log_path);
    } else {
        println!("[fetching::init_fetcher] no log file found");
    }
}

fn start_fetcher(app: AppHandle, interval_ms: Arc<AtomicU64>, log_path: PathBuf) {
    let log_pattern = Regex::new(r"\[\d{2}:\d{2}:\d{2}\] \[Client thread/INFO\]: \[CHAT\] ONLINE: (.*)$").unwrap();

    spawn(async move {
        let mut last_position = 0;

        if let Ok(at_start) = std::fs::metadata(&log_path) {
            last_position = at_start.len();
        }

        loop {
            if let Ok(mut file) = File::open(&log_path) {
                let current_len = file.metadata().map(|m| m.len()).unwrap_or(0);

                if current_len < last_position {
                    last_position = 0;
                }

                if current_len > last_position {
                    let _ = file.seek(SeekFrom::Start(last_position));
                    let reader = BufReader::new(file);

                    for line_res in reader.lines() {
                        if let Ok(line) = line_res {
                            println!("{}", line);
                        }
                    }
                }
            } else {
                println!("[fetching::start_fetcher] log file not found");
                break;
            }

            let sleep_duration = interval_ms.load(Ordering::Relaxed);
            sleep(Duration::from_millis(sleep_duration)).await;
        }
    });
}