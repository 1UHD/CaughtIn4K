use std::{env, fs, io};
use std::path::PathBuf;
use std::io::{Write, Read};
use serde::{Serialize, Deserialize};
use serde_json::from_str;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Settings {
    pub general: GeneralSettings,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct GeneralSettings {
    pub caching: bool,
    pub interval_ms: u64,
    pub client: String,
}

impl Settings {
    pub fn load(path: PathBuf) -> Self {
        return fs::read_to_string(path)
            .ok()
            .and_then(|content| serde_json::from_str(&content).ok())
            .unwrap_or_default();
    }


}


pub fn get_config_dir() -> Option<PathBuf> {
    if cfg!(target_os = "windows") {
        return env::var_os("APPDATA").map(PathBuf::from);
    } else if cfg!(target_os = "macos") {
        env::var_os("HOME").map(|home| {
            let mut path = PathBuf::from(home);
            path.push("Library");
            path.push("Application Support");
            return path;
        })
    } else {
        env::var_os("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                env::var_os("HOME").map(|home| {
                    let mut path = PathBuf::from(home);
                    path.push(".config");
                    return path;
                })
            })
    }
}

fn init_api_key(path: PathBuf) {
    let api_key_path = path.join("apikey.txt");
    let gitignore_path = path.join(".gitignore");

    if !api_key_path.exists() {
        let _ = fs::File::create(api_key_path).map_err(|e|{
            println!("[config::init_api_key] Couldn't create api key file: {}", e);
            return;
        });
        println!("[config::init_api_key] Created apikey.txt");
    } else {
        println!("[config::init_api_key] apikey.txt already exists");
    }

    if !gitignore_path.exists() {
        if let Ok(mut gitignore_file) = fs::File::create(gitignore_path) {
            let _ = gitignore_file.write_all("apikey.txt".as_bytes()).map_err(|e| {
                println!("[config::init_api_key] Couldn't write to gitignore: {}", e);
                return;
            });
            println!("[config::init_api_key] Created .gitignore");
        } else {
            println!("[config::init_api_key] Couldn't create .gitignore");
        }

    } else {
        println!("[config::init_api_key] .gitignore already exists");
    }
}

fn init_settings(path: PathBuf) {
    let settings_path = path.join("settings.json");


}

pub fn read_api_key() -> io::Result<String> {
    let config_dir_path = get_config_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Config directory not found")
    })?;

    let mut path = config_dir_path;
    path.push("caughtin4k");
    path.push("apikey.txt");

    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    return Ok(content);
}

pub fn write_api_key(content: String) -> io::Result<()> {
    let config_dir_path = get_config_dir().ok_or_else(|| {
        std::io::Error::new(std::io::ErrorKind::Other, "Config directory not found")
    })?;

    let mut path = config_dir_path;
    path.push("caughtin4k");
    path.push("apikey.txt");

    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}


pub fn init_config_system() {
    let config_dir_path = get_config_dir();
    if config_dir_path.is_none() {
        // Handle this shit
        println!("[config::init_config_system] FATAL: config_dir_path not found");
        return;
    }

    let config_path = config_dir_path.unwrap().join("caughtin4k");

    if !config_path.exists() {
        let _ = fs::create_dir(&config_path).map_err(|e|{
            println!("[config::init_config_system] Couldn't create config directory: {}", e);
            return;
        });
        println!("[config::init_config_system] Config path has been created");
    } else {
        println!("[config::init_config_system] Config path already exists.");
    }

    init_api_key(config_path);
}
