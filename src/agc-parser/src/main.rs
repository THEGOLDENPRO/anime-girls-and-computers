/* This binary is meant to be ran inside of the anime girls and computers repo. (root of the repo) */

use std::{fs, path::{Path, PathBuf}, process::exit};    

use imagesize;
use glob::glob;
use colored::*;
use regex::Regex;

enum LogLevel {
    Success,
    Failure,
    Warning,
}

fn main() {
    let mut return_code: u8 = 0;

    let uwu_ignore_file = fs::read_to_string(".uwuignore").expect("Can't find .uwuignore file!");
    let ignored_folders: Vec<&str> = uwu_ignore_file.lines().collect();

    let glob_demon = glob("**/*.png").unwrap()
        .chain(glob("**/*.jpg").unwrap())
        .chain(glob("**/*.jpeg").unwrap())
        .chain(glob("**/*.webp").unwrap());

    for file in glob_demon {
        let file_path = file.expect("Failed to retrieve PathBuf!");
        let file_parent_path = file_path.parent().unwrap();

        if ignored_folders.contains(&file_parent_path.file_name().unwrap().to_str().unwrap()) {
            continue;
        }

        let image_name = (&file_path).file_stem().unwrap().to_str().unwrap();

        // println!("Parsing / checking '{}'...", image_id);

        let (result, message) = check_image(&file_path, file_parent_path);

        match result {
            LogLevel::Success => {
                println!("[{}] ({}) {}", "PASS".bright_green(), image_name.bright_black(), message);
            }
            LogLevel::Warning => {
                println!("[{}] ({}) {}", "WARNING".yellow(), image_name.bright_black(), message);
            }
            LogLevel::Failure => {
                println!("[{}] ({}) {}", "FAILED".red().bold(), image_name.bright_black(), message);
                return_code = 1;
            }
        }
    }

    exit(return_code as i32);
}

fn check_image(image_path: &PathBuf, parent_path: &Path) -> (LogLevel, String) {
    let image_name = image_path.file_stem().unwrap().to_str().unwrap();

    let allowed_image_name_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

    if !allowed_image_name_regex.is_match(image_name) {
        return (LogLevel::Failure, format!("Such an image name is not allowed --> '{}'!", image_name));
    }

    let toml_path = parent_path.join(format!("{}.toml", image_name));

    if !toml_path.exists() {
        return (LogLevel::Failure, format!("TOML config does not exist! {:?} should be present.", toml_path));
    }

    match check_toml(&toml_path) {
        Ok(_) => {},
        Err(err) => {
            return (LogLevel::Failure, format!("TOML config is invalid: {:?}", err));
        }
    }

    let image_size = image_path.metadata().unwrap().len() as f64 / (1024.0 * 1024.0);

    if image_size >= 7.0 {
        return (
            LogLevel::Warning, 
            format!(
                "File size is over 7 MiB, the current file is size: {:.2} MiB. \
                \nJust remember the limit is 10 MiB and if above that we may not except your image.", 
                image_size
            )
        );
    }

    let image_resolution = imagesize::size(image_path).expect(format!("Failed to read the actual image of '{}'!", image_name).as_str());

    if !(image_resolution.width >= 1080 || image_resolution.height >= 1080) {
        return (LogLevel::Failure, "Image is not 1080p or above, it must be higher to fit our quality criteria.".to_string());
    }

    return (LogLevel::Success, "Passed all checks!".to_string());
}

fn check_toml(toml_path: &PathBuf) -> Result<toml::Value, toml::de::Error> {
    let toml_text = std::fs::read_to_string(toml_path).unwrap();

    let toml: Result<toml::Value, toml::de::Error> = toml::from_str(&toml_text);

    toml
}
