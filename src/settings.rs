use config;
use dirs;
use std::path::PathBuf;
use xdg::BaseDirectories;


pub fn read_settings(config_path: PathBuf, verbose: u16) -> Option<config::Config> {
    let mut settings = config::Config::default();
    match settings.merge(config::File::from(config_path)) {
        Ok(config) => Some(config.clone()),
        Err(e) => {
            if verbose > 0 {
                println!("Error while reading the configuration: {}", e);
            }
            None
        }
    }
}

pub fn get_config_path(config_fname: &str) -> Option<PathBuf> {
    let base_directories = BaseDirectories::new().ok()?;
    let xdg_config = base_directories.find_config_file(config_fname);
    let mut config_in_home = dirs::home_dir()?;
    config_in_home.push(config_fname);
    xdg_config.or(Some(config_in_home))
}


pub fn black_list(config: Option<config::Config>) -> Vec<String> {
    if let Some(config) = config {
        if let Ok(blacklist_config) = config.get_array("blacklist") {
            let bl: Result<Vec<String>, _> =
                blacklist_config.into_iter().map(|v| v.into_str()).collect();
            if let Ok(black_list) = bl {
                return black_list;
            }
        }
    }
    return vec![];
}

