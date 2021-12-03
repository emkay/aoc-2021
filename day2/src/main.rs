use std::fs;
use std::path::Path;

fn main() {
    let filepath = Path::new("./data/day2-input.txt");

    let s = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");

    let v: Vec<Vec<&str>> = s
        .split_terminator("\n")
        .map(|x| x.split(" ").collect())
        .collect();

    let mut horizontal_pos = 0;
    let mut depth_pos = 0;

    for item in v.iter() {
        let command = item[0];
        let value = item[1].parse::<u32>().unwrap();

        match command {
            "forward" => horizontal_pos += value,
            "down" => depth_pos += value,
            "up" => depth_pos -= value,
            _ => panic!("nope")
        }
    }

    let answer = horizontal_pos * depth_pos;
    println!("answer: {}", answer);
}
