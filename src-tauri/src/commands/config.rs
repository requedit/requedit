use crate::config;


#[tauri::command]
pub(crate) fn get_config() -> config::Config {
    config::get_global_config()
}
