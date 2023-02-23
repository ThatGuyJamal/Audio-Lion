use crate::helpers::{self, configuration};

#[tauri::command]
pub fn view_app_config(app_handle: tauri::AppHandle)-> Result<configuration::AppConfig, String>  {
    match helpers::configuration::read_config_file(app_handle) {
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
pub fn reset_app_config(app_handle: tauri::AppHandle) -> bool  {
    match helpers::configuration::delete_config_file(&app_handle) {
        // If the configuration file was deleted successfully, create a new one
        true => {
            // Clone app_handle and pass it to create_config_file
            let app_handle_clone = app_handle.clone();
            match helpers::configuration::create_config_file(app_handle_clone) {
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
