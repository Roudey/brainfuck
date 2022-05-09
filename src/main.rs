use std::{fs::{read_to_string}, env::{args}};

#[allow(arithmetic_overflow)]
fn main() {
    let mut memory = vec![0u8; 30000];
    let mut pointer = 0;

    for instruction in read_file().chars() {
        match instruction {
            '>' => pointer += 1,
            '<' => pointer -= 1,
            '+' => memory[pointer] += 1,
            '-' => memory[pointer] -= 1,
            '.' => print!("{}", memory[pointer] as char),
            ',' => memory[pointer] = get_input() as u8,
            '[' => {
                
            }
            ']' => {
                
            }
            _ => {}
        }
    }
}

fn read_file() -> String {
    let path = args().nth(1).expect("No file path provided");
    let contents = read_to_string(path).expect("Unable to read file");
    contents
}

fn get_input() -> char {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.chars().next() {
        Some(c) => c,
        None => get_input(),
    }
}