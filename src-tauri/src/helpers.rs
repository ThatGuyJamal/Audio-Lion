#![allow(dead_code)]

pub mod configuration {

    use cpal::traits::{DeviceTrait, HostTrait};
    use serde::{Deserialize, Serialize};
    use std::fs::File;
    use std::io::{prelude::*, ErrorKind};

    #[derive(Serialize, Deserialize, Debug)]
    /// The configuration file for the application
    pub struct AppConfig {
        pub audio_directories: Vec<String>,
        /// A vector of audio file types to search for, e.g. MP3, WAV, etc.
        pub audio_file_types_allowed: Vec<String>,
        // The name of the audio device to use for playback
        pub audio_device_name: Option<String>,
    }

    /// Creates a configuration file in the application's config directory
    ///
    /// Returns `Ok(())` if the file was created successfully
    ///
    /// Returns `Err` if the file was not created successfully
    pub async fn create_config_file(
        app_handle: tauri::AppHandle,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config = AppConfig {
            audio_directories: vec![],
            audio_file_types_allowed: vec![String::from("mp3"), String::from("wav")],
            audio_device_name: None,
        };
        let config_json = serde_json::to_string(&config)?;

        if let Some(config_path) = app_handle.path_resolver().app_config_dir() {
            // Create the directory if it doesn't exist
            std::fs::create_dir_all(&config_path)?;

            // Write the config.json file to the config directory and handle errors
            let file_path = config_path.join("config.json");
            match std::fs::write(&file_path, config_json) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unable to find writable config directory",
            )))
        }
    }

    /// Reads the configuration file and returns an `AppConfig` object
    ///
    /// Returns `Ok(AppConfig)` if the file was read successfully
    ///
    /// Returns `Err` if the file was not read successfully
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
                tauri::async_runtime::block_on(create_config_file(app_handle)).unwrap();
                File::open(&config_file_path)?
            }
            Err(err) => return Err(Box::new(err)),
        };

        let mut config_json = String::new();
        file.read_to_string(&mut config_json)?;

        // Serialize the `AppConfig` object to JSON
        let config: AppConfig = serde_json::from_str(&config_json)?;

        Ok(config)
    }

    /// Deletes the configuration file
    ///
    /// Returns `true` if the file was deleted successfully
    ///
    /// Returns `false` if the file was not deleted successfully
    pub async fn delete_config_file(app_handle: &tauri::AppHandle) -> bool {
        match app_handle.path_resolver().app_config_dir() {
            Some(config_path) => match std::fs::remove_file(config_path.join("config.json")) {
                Ok(_) => true,
                Err(_) => false,
            },
            None => false,
        }
    }

    #[derive(Debug)]
    pub enum AudioOutputError {
        OpenStreamError,
        PlayStreamError,
        StreamClosedError,
    }

    /// Updates the configuration file with the new configuration
    ///
    /// Returns `Ok(())` if the file was updated successfully
    ///
    /// Returns `Err` if the file was not updated successfully
    pub async fn update_config_file(
        app_handle: &tauri::AppHandle,
        config: &AppConfig,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let config_json = serde_json::to_string(&config)?;

        if let Some(config_path) = app_handle.path_resolver().app_config_dir() {
            let file_path = config_path.join("config.json");
            match std::fs::write(&file_path, config_json) {
                Ok(_) => Ok(()),
                Err(e) => Err(Box::new(e)),
            }
        } else {
            Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                "Unable to find writable config directory",
            )))
        }
    }

    /// Returns the default audio output device and its default stream configuration
    pub fn selected_device(
        app_handle: tauri::AppHandle,
    ) -> Result<(cpal::Device, cpal::SupportedStreamConfig), AudioOutputError> {
        // get the current config information
        let config = read_config_file(app_handle).unwrap();

        // Get the default audio output device.
        let host = cpal::default_host();

        let device = if let Some(device_name) = config.audio_device_name {
            host.output_devices()
                .unwrap()
                .find(|device| device.name().unwrap() == device_name)
                .unwrap()
        } else {
            match host.default_output_device() {
                Some(device) => {
                    println!(
                        "Using default audio output device: {}",
                        device.name().unwrap()
                    );
                    device
                }
                _ => {
                    eprintln!("Failed to get default audio output device");
                    return Err(AudioOutputError::OpenStreamError);
                }
            }
        };

        // Get the default stream configuration for the device.
        let config = device
            .default_output_config()
            .map_err(|_| AudioOutputError::OpenStreamError)?;

        println!("Using audio format: {:?}", config.sample_format());
        println!("Using sample rate: {:?}", config.sample_rate());

        Ok((device, config))
    }
}

pub mod player {
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Debug, Serialize, Deserialize)]
    pub enum AudioFileTypes {
        MP3,
        WAV,
    }

    impl AudioFileTypes {
        /// Returns an `AudioFileTypes` object from a file extension
        pub fn from_extension(extension: &str) -> Option<Self> {
            match extension.to_lowercase().as_str() {
                "mp3" => Some(Self::MP3),
                "wav" => Some(Self::WAV),
                _ => None,
            }
        }

        fn convert_audio_type_to_str(file_type: AudioFileTypes) -> &'static str {
            match file_type {
                AudioFileTypes::MP3 => "mp3",
                AudioFileTypes::WAV => "wav",
            }
        }

        fn convert_audio_type_from_str(file_type: String) -> Option<AudioFileTypes> {
            match file_type.to_lowercase().as_str() {
                "mp3" => Some(AudioFileTypes::MP3),
                "wav" => Some(AudioFileTypes::WAV),
                _ => None,
            }
        }
    }

    /// Returns a vector of `PathBuf` objects for all audio files in the given directory
    pub fn get_audio_files(dir: &str, file_type: AudioFileTypes) -> Vec<std::path::PathBuf> {
        let mut audio_files = vec![];

        if let Ok(entries) = std::fs::read_dir(dir) {
            for entry in entries {
                if let Ok(entry) = entry {
                    let path = entry.path();
                    // Check if the path is a file
                    if let Some(extension) = path.extension() {
                        // Check if the file type is the same as the file type we're looking for
                        if let Some(file_type_from_ext) =
                            AudioFileTypes::from_extension(&extension.to_string_lossy())
                        {
                            // If the file type is the same as the file type we're looking for, add it to the vector
                            if file_type_from_ext == file_type {
                                audio_files.push(path.to_path_buf());
                                // println!("Added file: {}", path.display());
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

        return audio_files;
    }
}
