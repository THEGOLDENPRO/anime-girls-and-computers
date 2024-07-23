/* This binary is meant to be ran inside of the anime girls and computers repo. (root of the repo) */

use std::fs;
use glob::glob;

fn main() {
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

        let image_path = file_path;
        let image_id = image_path.file_name().unwrap().to_str().unwrap();

        println!("Parsing / checking '{}'...", image_id);

        // TODO: Name check (is following convention), etc etc
    }
}