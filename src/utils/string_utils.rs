use std::fs::File;
use std::io::prelude::*;

pub fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

pub fn get_external_json(file_path: &str) -> String {
    let mut file = File::open(file_path).expect(&(format!("File {} not found", &file_path)));
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    return data;
}
