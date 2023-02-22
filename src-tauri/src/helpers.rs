// disable dead code warnings for this module
#![allow(dead_code)]

pub mod utilities {

    #[derive(PartialEq)]
    pub enum AudioFileTypes {
        MP3,
        WAV,
    }

    impl AudioFileTypes {
        fn from_extension(extension: &str) -> Option<Self> {
            match extension.to_lowercase().as_str() {
                "mp3" => Some(Self::MP3),
                "wav" => Some(Self::WAV),
                _ => None,
            }
        }
    }

    pub fn get_audio_files(dir: &str, file_type: AudioFileTypes) -> Vec<std::path::PathBuf> {
        let mut audio_files = vec![];

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    if let Some(extension) = path.extension() {
                        if let Some(file_type_from_ext) =
                            AudioFileTypes::from_extension(&extension.to_string_lossy())
                        {
                            if file_type_from_ext == file_type {
                                audio_files.push(path.to_path_buf());
                            } else {
                                continue;
                            }
                        }
                    } else {
                        continue;
                    }
                } else {
                    continue;
                }
            }
        }

        audio_files
    }
}

pub mod configuration {

    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{prelude::*, ErrorKind};

    #[derive(Serialize, Deserialize, Debug)]
    /// The configuration file for the application
    pub struct AppConfig {
        /// Whether or not the cache is enabled in the application
        pub cache_enabled: bool, // default: true
    }

    pub fn create_config_file(
        app_handle: tauri::AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = AppConfig {
            cache_enabled: true,
        };
        let config_json = serde_json::to_string(&config)?;

        if let Some(config_path) = app_handle.path_resolver().app_config_dir() {
            // Create the directory if it doesn't exist
            std::fs::create_dir_all(&config_path)?;

            // Write the config.json file to the config directory and handle errors
            let file_path = config_path.join("config.json");
            match std::fs::write(&file_path, config_json) {
                Ok(_) => {
                    // The file was created successfully
                    println!("Config file created successfully at {:?}", file_path);
                    Ok(())
                }
                Err(e) => {
                    // Handle the error here
                    println!("Error: {:?}", e);
                    Err(Box::new(e))
                }
            }
        } else {
            // The app does not have access to a writable config directory
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unable to find writable config directory",
            )))
        }
    }

    /// Reads the configuration file and returns an `AppConfig` object
    pub fn read_config_file(
        app_handle: tauri::AppHandle,
    ) -> Result<AppConfig, Box<dyn std::error::Error>> {
        let config_file_path = app_handle
            .path_resolver()
            .app_config_dir()
            .ok_or_else(|| {
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    "Unable to find writable config directory",
                )
            })?
            .join("config.json");

        let mut file = match File::open(&config_file_path) {
            Ok(file) => file,
            Err(ref err) if err.kind() == ErrorKind::NotFound => {
                // If the file is not found, create it using the `create_config_file` function
                let _ = create_config_file(app_handle)?;
                File::open(&config_file_path)?
            }
            Err(err) => return Err(Box::new(err)),
        };

        // The file exists, so read it into a string and deserialize it
        let mut config_json = String::new();
        file.read_to_string(&mut config_json)?;

        // Serialize the `AppConfig` object to JSON
        let config: AppConfig = serde_json::from_str(&config_json)?;

        Ok(config)
    }

    /// Deletes the configuration file
    /// Returns `true` if the file was deleted successfully
    /// Returns `false` if the file was not deleted successfully
    pub fn delete_config_file(app_handle: &tauri::AppHandle) -> bool {
    match app_handle.path_resolver().app_config_dir() {
        Some(config_path) => {
            match std::fs::remove_file(config_path.join("config.json")) {
                Ok(_) => true,
                Err(_) => false,
            }
        }
        None => false,
    }
}

}
