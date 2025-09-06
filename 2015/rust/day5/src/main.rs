use std::fs;

const INPUT_PATH: &str = "./inputs";
const DAY: &str = "input";

fn nice_list(input: std::str::Lines<'_>) -> i32 {
    let mut count: i32 = 0;

    for l in input {
        // ab, cd, pq, or xy
        if l.contains("ab") || l.contains("cd") || l.contains("pq") || l.contains("xy") {
            continue;
        }

        let mut c: Vec<u8> = Vec::from(l);

        // Urgh... count the vowels, I guess 🙄
        let vowel_count = c
            .iter()
            .filter(|&&x| x == b'a' || x == b'e' || x == b'i' || x == b'o' || x == b'u')
            .count();
        if vowel_count < 3 {
            continue;
        }

        // SORT -> DEDUP (check for double chars)
        let before_matches = c.len();
        c.dedup();
        if c.len() == before_matches {
            continue;
        }

        // -> REMOVE CONSONANTS (should give us vowel count)
        // I AM SO SAD THIS WAS SO NICE 😭 (basically got to re-use this anyway but still...)
        // c.sort();
        // c.dedup();
        // let uniq_vowel_count = c
        //     .iter()
        //     .filter(|&&x| x == b'a' || x == b'e' || x == b'i' || x == b'o' || x == b'u').count();

        count += 1;
    }
    count
}

// TODO: Use .scan() for this
// fn super_nice_list(input: std::str::Lines<'_>) -> i32 {
//     let mut count: i32 = 0;
//
//     for l in input {
//         let mut c: Vec<u8> = Vec::from(l);
//
//         let odd_chars = c.iter().fold(0, |acc, x| acc % 2 == 1);
//         let even_chars = c.iter().filter(|x| x % 2 == 0).collect();
//     }
//     1
// }

fn main() {
    let file: String = fs::read_to_string(format!("{}/{}.txt", INPUT_PATH, DAY))
        .expect("Failed to read input file");

    let nl_count = nice_list(file.lines());
    // let snl_count = super_nice_list(file.lines());
    //
    println!("Number of nice list: {}", nl_count.to_string());
    // println!("Number of super nice list: {}", snl_count.to_string());
}
