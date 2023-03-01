#![allow(dead_code)]

pub mod core {
    use anyhow::{anyhow, Result};
    use rodio::{OutputStream, Sink};
    use tauri::{App, Manager};
    use tokio_util::task::LocalPoolHandle;

    use crate::helpers;

    use super::stream::{self, AudioCommandResult};

    // Invokes the Audio Resource Manager for Audio Lion
    pub fn init(app: &mut App) {
        load_config(app.app_handle())
    }

    fn load_config(app_handle: tauri::AppHandle) {
        match helpers::configuration::read_config_file(app_handle) {
            Ok(_config) => {}
            Err(_) => {}
        }
    }

    /// # Commands
    /// - Play
    /// - Pause
    /// - Resume
    /// - Stop
    pub enum AudioCommands {
        Play,
        Pause,
        Resume,
        Stop,
    }

    /// ## Handles audio commands
    /// - `command`: The command to execute
    /// - `play_params`: The parameters to use when playing audio
    pub async fn handle_audio(
        command: AudioCommands,
        play_params: Option<stream::PlayAudioParams>,
    ) -> Result<AudioCommandResult> {
        // see https://docs.rs/tokio-util/latest/tokio_util/task/struct.LocalPoolHandle.html
        let pool = LocalPoolHandle::new(2);

        let output = pool
            .spawn_pinned(|| async move {
                let (_stream, stream_handle) =
                    OutputStream::try_default().map_err(anyhow::Error::msg)?;

                let sink = Sink::try_new(&stream_handle).map_err(anyhow::Error::msg)?;

                match command {
                    AudioCommands::Play => {
                        if let Some(params) = play_params {
                            let result = stream::play_audio(params, &sink)
                                .await
                                .map_err(|e| anyhow!(e.to_string()))?;
                            Ok(result)
                        } else {
                            Err(anyhow!("Play command called without play_params"))
                        }
                    }
                    AudioCommands::Pause => {
                        let result = stream::pause_audio(&sink)
                            .await
                            .map_err(|e| anyhow!(e.to_string()))?;
                        Ok(result)
                    }
                    AudioCommands::Resume => {
                        let result = stream::resume_audio(&sink)
                            .await
                            .map_err(|e| anyhow!(e.to_string()))?;
                        Ok(result)
                    }
                    AudioCommands::Stop => {
                        let result = stream::stop_audio(&sink)
                            .await
                            .map_err(|e| anyhow!(e.to_string()))?;
                        Ok(result)
                    }
                }
            })
            .await
            .unwrap();

        return output
    }
}

pub mod stream {
    use anyhow::{anyhow, Result};
    use rodio::Sink;
    use serde::{Deserialize, Serialize};
    use std::io::BufReader;

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

    #[derive(Debug, Serialize, Deserialize)]
    pub struct AudioCommandResult {
        pub success: bool,
        pub is_paused: bool,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlayAudioParams {
        pub file_path: String,
        pub file_type: String,
        pub file_index: usize,
    }

    /// Plays an audio file
    pub async fn play_audio(params: PlayAudioParams, sink: &Sink) -> Result<AudioCommandResult>
    where
        PlayAudioParams: Send + Sync + 'static,
    {
        let PlayAudioParams {
            file_path,
            file_type,
            file_index,
        } = params;

        let audio_file_type = match AudioFileTypes::convert_audio_type_from_str(file_type) {
            Some(file_type) => file_type,
            None => return Err(anyhow!("Invalid file type")), // invalid file type, return error
        };

        let audio_files = get_audio_files(&file_path, audio_file_type);

        let file_index = match audio_files.get(file_index) {
            Some(file_index) => file_index,
            None => return Err(anyhow!("Invalid file index")),
        };

        let file = std::fs::File::open(file_index)?;
        let decoder = rodio::Decoder::new(BufReader::new(file))?;

        sink.append(decoder);

        sink.sleep_until_end();

        Ok(AudioCommandResult {
            success: true,
            is_paused: sink.is_paused(),
        })
    }

    /// Pauses the currently playing audio file
    ///
    /// Returns true if the audio file was paused successfully, false otherwise
    pub async fn pause_audio(sink: &Sink) -> Result<AudioCommandResult, Box<dyn std::error::Error>> {
        sink.pause();

        Ok(AudioCommandResult {
            success: true,
            is_paused: sink.is_paused(),
        })
    }

    /// Resumes the currently paused audio file
    ///
    /// Returns true if the audio file was resumed successfully, false otherwise
    pub async fn resume_audio(
        sink: &Sink,
    ) -> Result<AudioCommandResult, Box<dyn std::error::Error>> {
        sink.play();

        Ok(AudioCommandResult {
            success: true,
            is_paused: sink.is_paused(),
        })
    }

    /// Stops the currently playing audio file and plays the next audio file in the queue
    ///
    /// Returns true if the audio file was stopped successfully, false otherwise
    pub async fn skip_audio(sink: &Sink) -> Result<AudioCommandResult, Box<dyn std::error::Error>> {
        sink.skip_one();

        Ok(AudioCommandResult {
            success: true,
            is_paused: sink.is_paused(),
        })
    }

    /// Empty's the audio queue and stops all music
    ///
    /// Returns true if the audio file was stopped successfully, false otherwise
    pub async fn stop_audio(sink: &Sink) -> Result<AudioCommandResult, Box<dyn std::error::Error>> {
        sink.stop();

        Ok(AudioCommandResult {
            success: true,
            is_paused: sink.is_paused(),
        })
    }

    /// Returns the number of audio files in the queue
    pub fn queue_size(sink: &Sink) -> usize {
        sink.len()
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
