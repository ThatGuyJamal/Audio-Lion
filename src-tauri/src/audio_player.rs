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
    use std::io::BufReader;

    use rodio::Source;
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

        fn convert_audio_type_from_str(file_type: &str) -> Option<AudioFileTypes> {
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

    /// Plays an audio file
    pub fn play_audio(file_path: &str, file_type: &str, file_index: usize) -> bool {
        let audio_file_type = match AudioFileTypes::convert_audio_type_from_str(file_type) {
            Some(file_type) => file_type,
            None => return false, // invalid file type, return false
        };

        println!("[play_audio] File type: {:?}", audio_file_type);
        println!("[play_audio] File index: {}", file_index);
        println!("[play_audio] File path: {}", file_path);

        let audio_files = get_audio_files(&file_path, audio_file_type);

        println!("[play_audio] Audio files: {:?}", audio_files);

        let file_index = match audio_files.get(file_index) {
            Some(file_index) => file_index,
            None => {
                println!("[play_audio] Invalid index: {}", file_index);
                return false;
            } // invalid index, return false
        };

        println!("[play_audio] Selected file: {}", file_index.display());

        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

        // Loads the audio file into a file buffer
        let file = match std::fs::File::open(file_index) {
            Ok(file) => file,
            Err(e) => {
                println!("[play_audio] Error opening file: {}", e);
                return false;
            } // error opening file, return false
        };

        // Decode that sound file into a source
        let decoder = match rodio::Decoder::new(BufReader::new(file)) {
            Ok(decoder) => decoder.convert_samples(),
            Err(e) => {
                println!("[play_audio] Error creating decoder: {}", e);
                return false;
            } // error creating decoder, return false
        };

        // Play the sound directly on the device
        stream_handle.play_raw(decoder);

        println!("[play_audio] Playing audio file: {}", file_index.display());

        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
        std::thread::sleep(std::time::Duration::from_secs(15));

        return true;
    }
}
