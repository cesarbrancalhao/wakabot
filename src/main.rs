use enigo::*;
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::thread::sleep;
use std::time::Duration;


fn find_file(directory: &Path, filename: &str) -> Option<PathBuf> {
    for entry in fs::read_dir(directory).expect("Directory not found.") {
        let entry = entry.expect("Error reading entry.");
        let path = entry.path();
        if path.is_file() && path.file_name().unwrap() == filename {
            return Some(path);
        } else if path.is_dir() {
            if let Some(found) = find_file(&path, filename) {
                return Some(found);
            }
        }
    }
    None
}

fn run_typing_process(file_path: PathBuf) {
    let file = File::open(&file_path).expect("Unable to open file.");
    let reader = BufReader::new(file);
    let mut enigo = Enigo::new();

    for line in reader.lines() {
        let line = line.expect("Unable to read line.");
        for char in line.chars() {
            match char {
                '_' => enigo.key_click(Key::Return),
                '\t' => enigo.key_click(Key::Tab),
                _ => {
                    sleep(Duration::from_millis(15000));
                    enigo.key_sequence(&char.to_string());
                }
            }
        }
    }
}

fn main() {
    let working_dir = env::current_dir().expect("Unable to get current directory.");
    let code_dir = working_dir.join("src");

    println!("Select language you want to farm.");
    println!("Type the extension only (eg. go, js, rs).");
    let mut selected_extension = String::new();
    io::stdin().read_line(&mut selected_extension).expect("Failed to read line.");
    let selected_extension = selected_extension.trim();

    let file_name = format!("{}.{}", selected_extension, selected_extension);
    let code_path = code_dir.clone();

    if let Some(found_file) = find_file(&code_path, &file_name) {
        println!("Found {:?}", found_file);
        println!("The typing will start in 7 seconds.");
        println!("The program will continue running until you press ctrl+c to stop.");

        sleep(Duration::from_millis(7000));

        loop {
            run_typing_process(found_file.clone());
        }
    } else {
        println!("File extension not supported.");
    }
}
