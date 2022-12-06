use std::fs::File;
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub fn input_file_path() -> PathBuf {
    input_file_path_with_name("input.txt")
}

pub fn input_file_path_with_name(filename: &str) -> PathBuf {
    PathBuf::from_str(filename).unwrap()
}

pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path> + std::fmt::Debug,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
