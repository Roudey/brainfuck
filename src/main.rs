use std::{fs::{self, read_to_string}, env::{self, args}};

#[derive(Debug)]
enum Instructions {
    PointerToRight,
    PointerToLeft,
    Increment,
    Decrement,
    Output,
    Input,
    LoopStart,
    LoopEnd,
}

//Brainfuck interpreter
fn main() {
    let contents = read_file();

    let mut memory = vec![0u8; 30000];
    let mut instructions: Vec<Instructions> = vec![];
    
    for i in contents.chars() {
        match i {
            '>' => instructions.push(Instructions::PointerToRight),
            '<' => instructions.push(Instructions::PointerToLeft),
            '+' => instructions.push(Instructions::Increment),
            '-' => instructions.push(Instructions::Decrement),
            '.' => instructions.push(Instructions::Output),
            ',' => instructions.push(Instructions::Input),
            '[' => instructions.push(Instructions::LoopStart),
            ']' => instructions.push(Instructions::LoopEnd),
            _ => {}
        }
    }

    let mut pointer = 0;
    for i in instructions {
        match i {
            Instructions::PointerToRight => pointer += 1,
            Instructions::PointerToLeft => pointer -= 1,
            Instructions::Increment => memory[pointer] += 1,
            Instructions::Decrement => memory[pointer] -= 1,
            Instructions::Output => print!("{}", memory[pointer] as char),
            Instructions::Input => memory[pointer] = get_input() as u8,
            _ => {}
        }
    }
}

fn read_file() -> String {
    //Get file path
    let path = args().nth(1).expect("No file path provided");
    //Read file to string
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