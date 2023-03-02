use std::{
    io::{stdin, BufReader},
    thread,
};

fn main() {
    let handle = thread::spawn(|| {
        // Get a output stream handle to the default physical sound device
        let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
        let sink = rodio::Sink::try_new(&handle).unwrap();

        // The path to the mp3 local files
        let dir = "resources/mp3";

        // Get a list of all the mp3 files in the directory
        let audio_files = get_audio_files(&dir, AudioFileTypes::MP3);

        // Display the mp3 files to the user
        println!("Audio files in {}: ", dir);
        for (i, mp3_file) in audio_files.iter().enumerate() {
            println!("{}: {}", i + 1, mp3_file.display());
        }

        // Prompt the user to select a file
        let file_index = get_file_selection(audio_files.len());

        // Log the file selection to the console
        let selected_file = &audio_files[file_index];
        println!("Selected file: {}", selected_file.display());

        let file = std::fs::File::open(selected_file).unwrap();
        let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();

        println!("Audio in sink: {:?}", sink.len());
        sink.append(decoder);
        println!("Audio in sink: {:?}", sink.len());

        let mut input = String::new();
        let stdin = stdin();
        // Wait for the audio to finish playing or for the user to type "stop"
        // Check if the user has typed "stop"
        loop {
            if stdin.read_line(&mut input).unwrap() > 0 && input.trim() == "stop" {
                println!("Stopping");
                println!("Audio in sink: {:?}", sink.len());
                sink.stop();
                println!("Stopped playing");
                println!("Audio in sink: {:?}", sink.len());
            }

            if stdin.read_line(&mut input).unwrap() > 0 && input.trim() == "skip" {
                println!("Skipping");
                println!("Audio in sink: {:?}", sink.len());
                if sink.len() > 0 {
                    println!("Skipping");
                    sink.skip_one();
                    println!("Skipped");
                } else {
                    println!("No more songs to skip");
                    println!("Audio in sink: {:?}", sink.len());
                }
            }

            if stdin.read_line(&mut input).unwrap() > 0 && input.trim() == "pause" {
                println!("Pausing");
                println!("Audio in sink: {:?}", sink.len());
                if sink.is_paused() {
                    println!("Already paused");
                } else {
                    sink.pause();
                    println!("Paused");
                    println!("Audio in sink: {:?}", sink.len());
                }
            }

            if stdin.read_line(&mut input).unwrap() > 0 && input.trim() == "resume" {
                println!("Resuming");
                println!("Audio in sink: {:?}", sink.len());
                if sink.is_paused() {
                    sink.play();
                    println!("Resumed");
                } else {
                    println!("Not paused");
                    println!("Audio in sink: {:?}", sink.len());
                }
            }

            if stdin.read_line(&mut input).unwrap() > 0 && input.trim() == "add" {
                println!("Adding");
                println!("Audio in sink{:?}", sink.len());
                let file = std::fs::File::open(selected_file).unwrap();
                let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
                sink.append(decoder);
                println!("Added");
                println!("Audio in sink: {:?}", sink.len());
            }
        }
    });

    handle.join().unwrap();
}

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

fn get_file_selection(num_files: usize) -> usize {
    loop {
        println!("Select a file (1-{}):", num_files);
        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim().parse::<usize>() {
            Ok(num) if num >= 1 && num <= num_files => return num - 1,
            _ => println!(
                "Invalid input. Please enter a number between 1 and {}",
                num_files
            ),
        }
    }
}
