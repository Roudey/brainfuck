use std::{fs::{read_to_string}, env::{args}};

enum Instructions {
    PointerLeft,
    PointerRight,
    Increment,
    Decrement,
    Output,
    Input,
    OpenBracket(usize), // index of the matching closing bracket
    CloseBracket(usize), // index of the matching opening bracket
}

#[allow(arithmetic_overflow)]
fn main() {
    let mut memory = vec![0u8; 30000];
    let mut pointer = 0;

    let char_buffer: Vec<char> = read_file().chars().collect();
    
    // Iterate over the instructions
    let mut input_buffer = Vec::new();
    for (i, instruction) in read_file().chars().enumerate() {
        match instruction {
            '>' => input_buffer.push(Instructions::PointerRight),
            '<' => input_buffer.push(Instructions::PointerLeft),
            '+' => input_buffer.push(Instructions::Increment),
            '-' => input_buffer.push(Instructions::Decrement),
            '.' => input_buffer.push(Instructions::Output),
            ',' => input_buffer.push(Instructions::Input),
            '[' => {
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
                input_buffer.push(Instructions::OpenBracket(index));
            },
            ']' => {
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
                input_buffer.push(Instructions::CloseBracket(index));
            }
            _ => {}
        }
    }

    let mut instruction_pointer = 0;

    // Execute the instructions from input_buffer
    while instruction_pointer < input_buffer.len() {
        match input_buffer[instruction_pointer] {
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