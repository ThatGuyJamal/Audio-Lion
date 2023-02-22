use std::io::{stdin, BufReader};

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    // The path to the mp3 local files
    let mp3_dir = "resources/mp3";

    // Get a list of all the mp3 files in the directory
    let mp3_files = get_mp3_files(mp3_dir);

    // Display the mp3 files to the user
    println!("MP3 files in {}: ", mp3_dir);
    for (i, mp3_file) in mp3_files.iter().enumerate() {
        println!("{}: {}", i + 1, mp3_file.display());
    }

    // Prompt the user to select a file
    let file_index = get_file_selection(mp3_files.len());

    // Log the file selection to the console
    let selected_file = &mp3_files[file_index];
    println!("Selected file: {}", selected_file.display());

    let file = std::fs::File::open(selected_file).unwrap();
    let decoder = rodio::Decoder::new(BufReader::new(file)).unwrap();
    sink.append(decoder);

    // Wait for the audio to finish playing or for the user to type "stop"
    loop {
        let mut input = String::new();
        let stdin = stdin();

        // Check if the user has typed "stop"
        if stdin.read_line(&mut input).unwrap_or(0) > 0 && input.trim() == "stop" {
            sink.stop();
            break;
        } else {
            println!("Type 'stop' to stop the audio")
        }

        // Check if the audio has finished playing
        if !sink.empty() {
            continue;
        }

        // If the audio has finished playing and the user has not typed "stop", the program ends
        break;
    }
}

fn get_mp3_files(dir: &str) -> Vec<std::path::PathBuf> {
    let mut mp3_files = vec![];

    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    if extension == "mp3" {
                        mp3_files.push(path.to_path_buf());
                    }
                }
            }
        }
    }

    mp3_files
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
