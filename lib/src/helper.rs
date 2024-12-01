use std::fs::File;
use std::io::{BufReader, Read};
use crate::error::Error;

pub fn parse_file_by_path(file_path: &str) -> Result<String, Error> {
    let path = std::env::current_dir().unwrap();
    let path = format!("{}{}", path.display(), file_path);
    
    let file = File::open(path).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();
    buf_reader.read_to_string(&mut contents).unwrap();
    Ok(contents)
}