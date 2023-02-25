#![allow(dead_code)]

pub mod core {
    use tauri::{App, Manager};

    use crate::helpers;

    // Invokes the Audio Resource Manager for Audio Lion
    pub fn init(app: &mut App) {
        println!("Audio Lion Core is now running!");

        // Load the configuration file
        load_config(app.app_handle());
    }

    fn load_config(app_handle: tauri::AppHandle) {
        // Load the configuration file
        match helpers::configuration::read_config_file(app_handle) {
            Ok(config) => {
                // Use the configuration data here
                println!("{:?}", config);
            }
            Err(e) => {
                // Handle the error here
                println!("Error: {:?}", e);
            }
        }
    }
}

pub mod stream {
    #[derive(PartialEq, Debug)]
    pub enum AudioFileTypes {
        MP3,
        WAV,
    }

    impl AudioFileTypes {
        /// Returns an `AudioFileTypes` object from a file extension
        fn from_extension(extension: &str) -> Option<Self> {
            match extension.to_lowercase().as_str() {
                "mp3" => Some(Self::MP3),
                "wav" => Some(Self::WAV),
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
                                println!("Added file: {}", path.display());
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
