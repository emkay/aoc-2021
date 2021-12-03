use std::fs;
use std::path::Path;

fn main() {
    let filepath = Path::new("./data/day3-input.txt");

    let s = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");

    let v: Vec<&str> = s
        .split_terminator("\n")
        .collect();

    let mut counter: Vec<i16> = vec![0; 12];
    for bin in v.iter() {
        for (i, c) in bin.chars().enumerate() {
            match c {
                '0' => counter[i] -= 1,
                '1' => counter[i] += 1,
                _ => {}
            }
        }
    }

    let mut gamma: Vec<&str> = Vec::new();
    let mut epsilon : Vec<&str> = Vec::new();

    for num in counter.iter() {
        if num > &0 {
            gamma.push("1");
            epsilon.push("0");
        } else {
            gamma.push("0");
            epsilon.push("1");
        }
    }
    let gamma_rate = isize::from_str_radix(&gamma.join(""), 2)
        .unwrap();
    let epsilon_rate = isize::from_str_radix(&epsilon.join(""), 2)
        .unwrap();

    let answer = gamma_rate * epsilon_rate;

    println!("{}", answer);
}
