use std::collections::{BTreeMap};
use std::ffi::OsStr;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path};

use walkdir::WalkDir;

pub fn retrieve_line_count_per_file(file_path: &str) -> BTreeMap<String, i32> {
    let path = Path::new(file_path);
    let mut file_data = BTreeMap::new();

    for entry in WalkDir::new(path) {
        if entry.is_ok() {
            let unwrapped_entry = entry.unwrap();
            let path = unwrapped_entry.path();
            let extension = path.extension().and_then(OsStr::to_str);
            if extension != None && path.is_file() {
                let lines = read_lines(path);

                let mut total_lines_in_file = 0;
                if lines.is_ok() {
                    for _ in lines.unwrap() {
                        total_lines_in_file += 1;
                    }
                }
                let key = String::from(extension.unwrap());
                let previous_line_count = *file_data.entry(key.clone()).or_insert(0);
                let new_line_count = previous_line_count + total_lines_in_file;
                file_data.insert(key, new_line_count);
            }
        }
    }

    return file_data;
}

fn read_lines<P>(path: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(path)?;
    Ok(io::BufReader::new(file).lines())
}