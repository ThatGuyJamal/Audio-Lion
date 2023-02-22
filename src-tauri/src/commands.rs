use crate::helpers;

#[tauri::command]
pub fn view_app_config()  {
    match helpers::configuration::read_config_file() {
        Ok(config) => {
            // Use the configuration data here
            println!("Cache Enabled: {:?}", config);
        }
        Err(e) => {
            eprintln!("Error reading config file: {}", e);
        }
    }
}
