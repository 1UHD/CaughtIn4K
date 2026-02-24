use std::{env, fs, io};
use std::path::PathBuf;
use std::io::{Write, Read};

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

    if !api_key_path.exists() {
        let _ = fs::File::create(api_key_path).map_err(|e|{
            println!("[config::init_api_key] Couldn't create api key file: {}", e);
        });
    }
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

    let mut file = fs::File::create(path)?; // create() truncates/overwrites
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
        });
    }

    init_api_key(config_path);
}
