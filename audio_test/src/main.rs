use std::io::BufReader;

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&handle).unwrap();

    // The path to the mp3 local files
    let mp3_dir = "resources/mp3";

    // Get a list of all the mp3 files in the directory
    let mp3_files = get_mp3_files(mp3_dir);

    // If there are no mp3 files in the directory, exit the program
    if mp3_files.is_empty() {
        println!("No mp3 files found in {}", mp3_dir);
        return;
    }

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

    let file = std::fs::File::open(format!("{}", selected_file.display())).unwrap();
    sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

    // Play the file until it is finished, then exit the program
    sink.sleep_until_end();
}

// Get a list of all the mp3 files in a directory
fn get_mp3_files(dir: &str) -> Vec<std::path::PathBuf> {
    let mut mp3_files = vec![];

    // Read the directory and get a list of all the files
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries {
            // If the entry is a file, check if it is an mp3 file
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(extension) = path.extension() {
                    // If the file is an mp3 file, add it to the list
                    // Otherwise, skip the file
                    if extension == "mp3" {
                        mp3_files.push(path.to_path_buf());
                    } else {
                        println!("Skipping file: {}", path.display());
                    }
                }
            }
        }
    }

    return mp3_files
}

// Prompt the user to select a file
// The user must enter a number between 1 and the number of files
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
