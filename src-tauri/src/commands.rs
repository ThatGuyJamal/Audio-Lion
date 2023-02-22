use crate::helpers::{self, configuration};

#[tauri::command]
pub fn view_app_config()-> Result<configuration::AppConfig, String>  {
    match helpers::configuration::read_config_file() {
        Ok(config) => {
            // Use the configuration data here
            // println!("{:?}", config);
            return Ok(config);
        }
        Err(e) => {
            // Handle the error here
            // println!("Error: {:?}", e);
            return Err(e.to_string());
        }
    }
}

#[tauri::command]
pub fn reset_app_config() -> bool  {
    match helpers::configuration::delete_config_file() {
        // If the configuration file was deleted successfully, create a new one
        true => {
            // Create a new configuration file, and return true if it was created successfully
            match helpers::configuration::create_config_file() {
                Ok(_) => {
                    return true;
                }
                Err(_) => {
                    // Handle the error here
                    // println!("Error: {:?}", e);
                    return false;
                }
            }
        }
        // If the configuration file was not deleted successfully, return false
        false => {
            return false;
        }
    }
}
