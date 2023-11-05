use std::{fs, io};

pub fn read_file(day: usize) -> Result<String, io::Error> {
    let path = format!("puzzle_inputs/day_{}.txt", day);
    fs::read_to_string(path)
}
