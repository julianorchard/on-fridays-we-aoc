use std::fs;

const INPUT_PATH: &str = "./inputs";
const DAY: &str = "input";

fn main() {
    let file: String = fs::read_to_string(format!("{}/{}.txt", INPUT_PATH, DAY))
        .expect("Failed to read input file");

    let mut paper: i32 = 0;
    let mut ribbon: i32 = 0;

    for l in file.lines() {
        //
        // Format is l*w*h
        //
        // Full calculation is:
        //
        // 2*l*w + 2*w*h + 2*h*l
        //
        let mut nums: Vec<i32> = l.split('x').map(|s| s.parse::<i32>().unwrap()).collect();

        // Dammit, this was so cool (but wrong):
        // let quick_maths: i32 = nums.iter().fold(1, |acc, n| acc * n);

        let numberios: Vec<i32> = [
            2 * nums[0] * nums[1],
            2 * nums[1] * nums[2],
            2 * nums[2] * nums[0],
        ]
        .to_vec();

        let maths: i32 = numberios.iter().sum();
        let smallest: i32 = *numberios.iter().min().unwrap() / 2;

        nums.sort(); // this feels sadge

        // Note: ::<i32>() - the above specifies the type explicitly!!!!
        let rib_num: i32 = [nums[0], nums[1]].into_iter().sum::<i32>() * 2;
        // Start at 1 because *
        // We did use it in the end lmao
        let bow_num: i32 = nums.iter().fold(1, |acc, x| acc * x);

        ribbon = ribbon + bow_num + rib_num;
        paper = paper + maths + smallest;
    }

    println!("Area of papier: {}", paper);
    println!("Length of the ribbons: {}", ribbon);
}
