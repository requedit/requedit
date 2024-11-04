use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use serde_yaml;
use shellexpand::tilde;
use std::{fs, path, sync::Mutex};

use crate::error::RequeditError;

static CONFIG: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(load().unwrap()));

pub(crate) fn get_global_config() -> Config {
    let config = CONFIG.lock().unwrap();
    config.clone()
}

pub(crate) fn load() -> Result<Config, RequeditError> {
    let workplace = tilde("~/.requedit").to_string();
    let config_name = tilde("~/.requedit/config.yaml").to_string();
    let workplace_path = path::Path::new(&workplace);
    if !workplace_path.exists() {
        fs::create_dir_all(workplace_path)?;
    }
    let config_path = path::Path::new(&config_name);
    let config = if config_path.exists() {
        let file = fs::File::open(config_path)?;
        let config_from_file: Config = serde_yaml::from_reader(file)?;
        config_from_file
    } else {
        let default_config = Config::default();
        let file = fs::File::create(&config_name)?;
        let yaml_config = serde_yaml::to_value(&default_config)?;
        serde_yaml::to_writer(file, &yaml_config)?;
        default_config
    };
    Ok(config)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub(crate) struct Config {
    pub(crate) cer_name: String,
    pub(crate) key_name: String,
    pub(crate) address: String,
    pub(crate) port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cer_name: tilde("~/.requedit/requedit.cer").to_string(),
            key_name: tilde("~/.requedit/requedit.key").to_string(),
            address: "127.0.0.1".to_string(),
            port: 9870_u16,
        }
    }
}
