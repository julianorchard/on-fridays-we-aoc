use std::fs;

const INPUT_PATH: &str = "./inputs";
const DAY: &str = "input";

fn s(c: char, mut x: i32, mut y: i32) -> (i32, i32) {
    match c {
        '^' => y = y + 1,
        'v' => y = y - 1,
        '>' => x = x + 1,
        '<' => x = x - 1,
        _ => {
            println!("Unexpected character: '{}'", c);
        }
    }

    return (x, y);
}

fn main() {
    let file: String = fs::read_to_string(format!("{}/{}.txt", INPUT_PATH, DAY))
        .expect("Failed to read input file");

    let (mut sum, mut sx, mut sy, mut rx, mut ry, mut uniq) = (0, 0, 0, 0, 0, 0);
    let mut visited: Vec<String> = Vec::new();

    let mut x: i32;
    let mut y: i32;

    for c in file.chars() {
        sum = sum + 1;

        if sum % 2 == 1 {
            // santa
            (x, y) = s(c, sx, sy);
            (sx, sy) = (x, y);
        } else {
            // robot santa
            (x, y) = s(c, rx, ry);
            (rx, ry) = (x, y);
        }

        if !visited.contains(&format!("{}, {}", x, y)) {
            visited.push(format!("{}, {}", x, y));
            uniq = uniq + 1;
        }
    }

    println!("Unique houses visited: {}", uniq)
}
