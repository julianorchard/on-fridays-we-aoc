use std::fs;

const INPUT_PATH: &str = "./inputs";
const DAY: &str = "input";

fn main() {
    let file: String = fs::read_to_string(format!("{}/{}.txt", INPUT_PATH, DAY))
        .expect("Failed to read input file");

    let north = '^';
    let south = 'v';
    let east = '>';
    let west = '<';

    // let (mut x, mut y) = (0, 0);
    // let mut uniq: i32 = 0;
    // let mut visited: Vec<String> = Vec::new();
    //
    // for c in file.chars() {
    //     if c == north {
    //         y = y + 1;
    //     } else if c == south {
    //         y = y - 1;
    //     } else if c == east {
    //         x = x + 1;
    //     } else if c == west {
    //         x = x - 1;
    //     } else {
    //         println!("Unexpected character: '{}'", c);
    //     }
    //
    //     if !visited.contains(&format!("{}, {}", x, y)) {
    //         visited.push(format!("{}, {}", x, y));
    //         uniq = uniq + 1;
    //     }
    // }
    //
    // println!("Unique houses visited: {}", uniq)

    let (mut sum, mut sx, mut sy, mut rx, mut ry, mut uniq) = (0, 0, 0, 0, 0, 0);
    let mut visited: Vec<String> = Vec::new();

    let mut x: i32;
    let mut y: i32;

    for c in file.chars() {
        sum = sum + 1;

        if sum % 2 == 1 {
            // santa
            if c == north {
                sy = sy + 1;
            } else if c == south {
                sy = sy - 1;
            } else if c == east {
                sx = sx + 1;
            } else if c == west {
                sx = sx - 1;
            } else {
                println!("Unexpected character: '{}'", c);
            }

            x = sx;
            y = sy;
        } else {
            // robot santa
            if c == north {
                ry = ry + 1;
            } else if c == south {
                ry = ry - 1;
            } else if c == east {
                rx = rx + 1;
            } else if c == west {
                rx = rx - 1;
            } else {
                println!("Unexpected character: '{}'", c);
            }

            x = rx;
            y = ry;
        }

        if !visited.contains(&format!("{}, {}", x, y)) {
            visited.push(format!("{}, {}", x, y));
            uniq = uniq + 1;
        }
    }

    println!("Unique houses visited: {}", uniq)
}
