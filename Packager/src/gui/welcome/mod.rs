use std::io::prelude::*;
use std::path::Path;
use std::fs::File;

pub mod welcome;
pub mod consts;

pub fn recent_path<'a>() -> Option<String> {
    let path_check = String::from("recent_project");
    let correct_path_to_json = Path::new(&path_check);
    if correct_path_to_json.exists() {
        if correct_path_to_json.is_file() {
            let mut file: File = File::open("recent_project").unwrap();
            let mut path_path = String::new();
            {
                file.read_to_string(&mut path_path);
            }
            let path_stupidity = path_path.clone();
            let path: &Path = Path::new(&path_stupidity);

            if path.exists() {
                if path.is_file() {

                    return Some(path_path);

                }
            }
        }
    }
    return None;
}

pub fn set_recent_path(path_location: String) {

    let mut file_ref = File::create("recent_project").unwrap();
    file_ref.write_all(path_location.as_str().as_bytes());

}