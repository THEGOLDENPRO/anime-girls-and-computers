/* This binary is meant to be ran inside of the anime girls and computers repo. (root of the repo) */

use std::{fs, path::{Path, PathBuf}, process::exit};
use colored::*;
use glob::glob;
use regex::Regex;

fn main() {
    let mut return_code: u8 = 0;

    let uwu_ignore_file = fs::read_to_string(".uwuignore").expect("Can't find .uwuignore file!");
    let ignored_folders: Vec<&str> = uwu_ignore_file.lines().collect();

    let glob_demon = glob("**/*.png").unwrap()
        .chain(glob("**/*.jpg").unwrap())
        .chain(glob("**/*.jpeg").unwrap())
        .chain(glob("**/*.webp").unwrap())
        .chain(glob("**/*.gif").unwrap());

    for file in glob_demon {
        let file_path = file.expect("Failed to retrieve PathBuf!");
        let file_parent_path = file_path.parent().unwrap();

        if ignored_folders.contains(&file_parent_path.file_name().unwrap().to_str().unwrap()) {
            continue;
        }

        let image_name = (&file_path).file_stem().unwrap().to_str().unwrap();

        // println!("Parsing / checking '{}'...", image_id);

        let (result, message) = check_image(&file_path, file_parent_path);

        if result == false {
            log_success(false, image_name, message.unwrap().as_str());
            return_code = 1;
            continue;
        }

        log_success(true, image_name, "Passed all checks!");
    }

    exit(return_code as i32);
}

fn log_success(successful: bool, image_id: &str, message: &str) {
    if successful {
        println!("[{}] ({}) {}", "PASS".bright_green(), image_id.bright_black(), message);
        return;
    }

    println!("[{}] ({}) {}", "FAILED".red().bold(), image_id.bright_black(), message)
}

fn check_image(image_path: &PathBuf, parent_path: &Path) -> (bool, Option<String>) {
    let image_name = image_path.file_stem().unwrap().to_str().unwrap();

    let allowed_image_name_regex = Regex::new(r"^[a-zA-Z0-9_]+$").unwrap();

    if !allowed_image_name_regex.is_match(image_name) {
        return (false, Some(format!("Such an image name is not allowed --> '{}'!", image_name)))
    }

    // TODO: more uwu checks

    let toml_path = parent_path.join(format!("{}.toml", image_name));

    if !toml_path.exists() {
        return (false, Some(format!("TOML config does not exist! {:?} should be present.", toml_path)));
    }

    return (true, None);
}