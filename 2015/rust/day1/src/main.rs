use std::fs;

const INPUT_PATH: &str = "./inputs";
const DAY: &str = "day1";

fn main() {
    let file: String = fs::read_to_string(format!("{}/{}.txt", INPUT_PATH, DAY))
        .expect("Failed to read input file");

    let mut floor: i32 = 0;
    let mut basement_instruction: i32 = 0;
    let mut instruction: i32 = 0;

    for c in file.chars() {
        instruction = instruction + 1;

        if c == '(' {
            floor = floor + 1;
        } else if c == ')' {
            floor = floor - 1;
        }

        if floor < 0 && basement_instruction == 0 {
            basement_instruction = instruction;
        }
    }

    println!("Travelled to floor {}", floor);

    println!(
        "Travelled to the basement at instruction {}",
        basement_instruction
    );
}
