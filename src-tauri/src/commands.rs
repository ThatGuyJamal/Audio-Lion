use crate::{
    audio_player::{self, stream::AudioFileTypes},
    helpers::{self, configuration},
};

#[tauri::command]
pub fn view_app_config(app_handle: tauri::AppHandle) -> Result<configuration::AppConfig, String> {
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
pub fn reset_app_config(app_handle: tauri::AppHandle) -> bool {
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

#[tauri::command]
pub fn set_app_config(
    app_handle: tauri::AppHandle,
    audio_directories: Vec<String>,
    audio_file_types_allowed: Vec<String>,
) -> bool {
    println!("set_app_config: {:?}", audio_directories);
    println!("set_app_config: {:?}", audio_file_types_allowed);

    let config = configuration::AppConfig {
        audio_directories,
        audio_file_types_allowed,
    };
    match helpers::configuration::update_config_file(&app_handle, &config) {
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

#[tauri::command]
pub fn get_audio_files(app_handle: tauri::AppHandle, audio_file_type: String) -> Vec<String> {
    let config = helpers::configuration::read_config_file(app_handle).unwrap();
    let result = audio_player::stream::get_audio_files(&config.audio_directories[0], AudioFileTypes::from_extension(&audio_file_type).unwrap());

    let mut audio_files: Vec<String> = Vec::new();

    for file in result {
        audio_files.push(file.display().to_string());
    }

    println!("get_audio_files {:?}", audio_files);

    return audio_files;
}

#[tauri::command]
pub fn play_audio_file(
    file_path: &str,
    file_type: &str,
    file_index: usize,
) -> bool {
    let result = audio_player::stream::play_audio(file_path, file_type, file_index);

    if result == true {
        return true;
    } else {
        return false;
    }
}
