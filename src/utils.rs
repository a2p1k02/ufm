use std::{env, fs};
use std::fs::File;
use std::path::Path;

pub fn show_hints() {
    print!("ls - show files | ");
    print!("cd - change directory | ");
    print!("cf - create file | ");
    print!("nd - create directory | ");
    print!("df - delete file | ");
    print!("dd - delete directory | ");
    print!("h - show keys | ");
    println!("q - exit");
}

pub fn check_command(command: &str, args: &str) {
    match command {
        "h" => show_hints(),
        "ls" => show_files(),
        "cd" => {
            if fs::metadata(args).is_ok() || args.is_empty() {
                change_directory(args);
            }
        },
        "nd" => {
            if !fs::metadata(args).is_ok() {
                create_directory(args);
            }
        },
        "dd" => {
            if fs::metadata(args).is_ok() {
                delete_directory(args);
            }
        },
        "df" => {
            if fs::metadata(args).is_ok() {
                delete_file(args);
            }
        },
        "cf" => {
            if !fs::metadata(args).is_ok() {
                create_file(args);
            }
        },
        _ => println!("Command not found!")
    }
}

pub fn change_directory(directory: &str) {
    if directory.is_empty() {
        env::set_current_dir(dirs::home_dir().unwrap()).unwrap();
    } else {
        env::set_current_dir(Path::new(directory)).unwrap();
    }
}

pub fn create_directory(directory: &str) {
    let path = "/".to_owned() + directory;
    fs::create_dir(env::current_dir().unwrap().display().to_string() + path.as_str()).unwrap();
}

pub fn create_file(filename: &str) {
    File::create(filename).unwrap();
}

pub fn delete_file(filename: &str) {
    let path = "/".to_owned() + filename;
    fs::remove_file(env::current_dir().unwrap().display().to_string() + path.as_str()).unwrap();
}

pub fn delete_directory(directory: &str) {
    let path = "/".to_owned() + directory;
    fs::remove_dir(env::current_dir().unwrap().display().to_string() + path.as_str()).unwrap();
}

pub fn show_files() {
    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        let path = path.unwrap().path();
        if path.is_file() {
            let file = path.display().to_string().replace("./", "");
            println!("{file} ");
        } else if path.is_dir() {
            let dir = path.display().to_string().replace("./", "");
            println!("{dir}/ ");
        }
    }
}