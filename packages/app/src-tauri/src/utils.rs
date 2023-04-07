#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

use crate::{config::AppConfig, types::AudioOutputError};
use serde::{Deserialize, Serialize};
use specta::Type;
use tauri::{Manager, App, Wry, CustomMenuItem, AppHandle};

#[derive(PartialEq, Debug, Serialize, Deserialize, Type, Clone)]
pub enum AudioFileTypes {
    MP3,
    WAV,
    WEBM,
}

impl AudioFileTypes {
    /// Returns an `AudioFileTypes` object from a file extension
    pub fn from_extension(extension: &str) -> Option<Self> {
        match extension.to_lowercase().as_str() {
            "mp3" => Some(Self::MP3),
            "wav" => Some(Self::WAV),
            "webm" => Some(Self::WEBM),
            _ => None,
        }
    }

    fn convert_audio_type_to_str(file_type: AudioFileTypes) -> &'static str {
        match file_type {
            AudioFileTypes::MP3 => "mp3",
            AudioFileTypes::WAV => "wav",
            AudioFileTypes::WEBM => "webm",
        }
    }

    fn convert_audio_type_from_str(file_type: String) -> Option<AudioFileTypes> {
        match file_type.to_lowercase().as_str() {
            "mp3" => Some(AudioFileTypes::MP3),
            "wav" => Some(AudioFileTypes::WAV),
            "webm" => Some(AudioFileTypes::WEBM),
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