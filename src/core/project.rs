extern crate dirs;

use std::fs;
use std::path::Path;
use crate::core::config;

const PROJECT_FOLDER: &str = "Schadenfreude";
const CONFIG_FILE: &str = ".config.toml";

pub fn load_project() {
    if has_config_folder() == false {
        create_config_folder();
    }

    if has_config_file() == false {
        create_config_file();
    }

    let mut config = config::Config::default();

    let home_dir = dirs::home_dir().unwrap();
    let config_file_path = Path::new(&home_dir).join(PROJECT_FOLDER).join(CONFIG_FILE);
    config.load(config_file_path);
}

fn has_config_folder() -> bool {
    return Path::new(&dirs::home_dir().unwrap()).join(PROJECT_FOLDER).is_dir();
}

fn create_config_folder() {
    fs::create_dir(Path::new(&dirs::home_dir().unwrap()).join(PROJECT_FOLDER));
}

fn has_config_file() -> bool {
    return Path::new(&dirs::home_dir().unwrap()).join(PROJECT_FOLDER).join(CONFIG_FILE).is_file();
}

fn create_config_file() {
    let home_dir = dirs::home_dir().unwrap();
    let config_file_path = Path::new(&home_dir).join(PROJECT_FOLDER).join(CONFIG_FILE);
    let mut config: config::Config = config::Config::default();
    config.create_new(config_file_path);
}