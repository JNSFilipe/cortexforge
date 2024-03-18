use std::fs;

pub fn read_file(file: &str) -> String {
    fs::read_to_string(file).expect("Unable to read file")
}

pub fn filter_chars(input: &str) -> String {
    input
        .chars()
        .filter(|&c| match c {
            '<' | '>' | '+' | '-' | '[' | ']' | '.' | ',' => true,
            _ => false,
        })
        .collect()
}
