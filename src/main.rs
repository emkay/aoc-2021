use std::fs;
use std::path::Path;

fn main() {
    let filepath = Path::new("./data/day-1-input.txt");

    let s = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");

    let v: Vec<u16> = s.split_terminator("\n")
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    let mut increase_count = 0;

    let iter = v.windows(2);
    for items in iter {
        let item = items[0];
        let next = items[1];

        if item <= next {
            increase_count += 1;
        }
    }

    println!("Increases: {}", increase_count);
}
