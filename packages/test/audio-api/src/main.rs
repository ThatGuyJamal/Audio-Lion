use rodio::{Decoder, OutputStream, Sink};
use std::{io::BufReader, thread, sync::{Arc, Mutex}};

fn main() {
    let file = select_audio_file();

    let (_stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Sink::try_new(&stream_handle).unwrap();

    let file = std::fs::File::open(format!("assets/{}", file)).unwrap();

    let source = Decoder::new(BufReader::new(file)).unwrap();

    sink.append(source);

    let sink = Arc::new(Mutex::new(sink));

    let sink_thread = sink.clone();

    let play_thread = thread::spawn(move || {
        let sink = sink_thread.lock().unwrap();

        println!("Playing");
        thread::sleep(std::time::Duration::from_secs(5));

        println!("Pausing");
        sink.pause();
        thread::sleep(std::time::Duration::from_secs(2));

        println!("Resuming");    
        sink.play();
        thread::sleep(std::time::Duration::from_secs(5));
    });

    play_thread.join().expect("Couldn't join on the associated thread!");

    println!("Done playing");
}

fn select_audio_file() -> String {
    let files = find_audio_files();
    let mut file = String::new();
    let mut input = String::new();
    while file.is_empty() {
        println!("Select a file to play:");
        for (i, file) in files.iter().enumerate() {
            println!("{}: {}", i + 1, file);
        }
        std::io::stdin().read_line(&mut input).unwrap();
        let index = input.trim().parse::<usize>().unwrap();
        if index > 0 && index <= files.len() {
            file = files[index - 1].clone();
        }
    }
    return file;
}

fn find_audio_files() -> Vec<String> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir("assets").unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_str().unwrap();
            if file_name.ends_with(".mp3") {
                files.push(file_name.to_string());
            }
        }
    }
    return files;
}
