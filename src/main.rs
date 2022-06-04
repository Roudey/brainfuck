use std::{fs::{read_to_string}, env::{args}};

enum Instructions {
    PointerLeft,
    PointerRight,
    Increment,
    Decrement,
    Output,
    Input,
    OpenBracket(usize), // Index of the matching closing bracket
    CloseBracket(usize), // Index of the matching opening bracket
}

#[allow(arithmetic_overflow)]
fn main() {
    let mut memory = vec![0u8; 30000];
    let mut pointer = 0;

    let char_buffer: Vec<char> = read_file().chars().collect();
    
    // Iterate over the instructions
    let mut instruction_buffer = Vec::new();
    for (i, instruction) in char_buffer.iter().enumerate() {
        match instruction {
            '>' => instruction_buffer.push(Instructions::PointerRight),
            '<' => instruction_buffer.push(Instructions::PointerLeft),
            '+' => instruction_buffer.push(Instructions::Increment),
            '-' => instruction_buffer.push(Instructions::Decrement),
            '.' => instruction_buffer.push(Instructions::Output),
            ',' => instruction_buffer.push(Instructions::Input),
            '[' => {
                // Push the index of the matching closing bracket
                let mut index = i;
                let mut count = 1;
                while count != 0 {
                    match char_buffer.get(index + 1) {
                        Some('[') => count += 1,
                        Some(']') => count -= 1,
                        None => break,
                        _ => {}
                    }
                    index += 1;
                }
                instruction_buffer.push(Instructions::OpenBracket(index));
            },
            ']' => {
                // Push the index of the matching opening bracket
                let mut index = i;
                let mut count = 1;
                while count != 0 {
                    match char_buffer.get(index - 1) {
                        Some('[') => count -= 1,
                        Some(']') => count += 1,
                        None => break,
                        _ => {}
                    }
                    index -= 1;
                }
                instruction_buffer.push(Instructions::CloseBracket(index));
            }
            _ => {}
        }
    }

    let mut instruction_pointer = 0;

    // Execute the instructions from the instruction buffer
    while instruction_pointer < instruction_buffer.len() {
        match instruction_buffer[instruction_pointer] {
            Instructions::PointerRight => if pointer > 29999 {
                pointer = 0;
            } else {
                pointer += 1;
            },
            Instructions::PointerLeft => if pointer == 0 {
                pointer = 29999;
            } else {
                pointer -= 1;
            },
            Instructions::Increment => memory[pointer] += 1,
            Instructions::Decrement => memory[pointer] -= 1,
            Instructions::Output => print!("{}", memory[pointer] as char),
            Instructions::Input => {
                memory[pointer] = get_input() as u8;
            }
            Instructions::OpenBracket(index) => {
                if memory[pointer] == 0 {
                    instruction_pointer = index;
                }
            }
            Instructions::CloseBracket(index) => {
                if memory[pointer] != 0 {
                    instruction_pointer = index;
                }
            }
        }
        instruction_pointer += 1;
    }
}

// Read file from the command line arguments
fn read_file() -> String {
    let path = args().nth(1).expect("No file path provided");
    let contents = read_to_string(path).expect("Unable to read file");
    contents
}

// Get input from stdin (trough the terminal)
fn get_input() -> char {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read input");
    match input.chars().next() {
        Some(c) => c,
        None => get_input(),
    }
}