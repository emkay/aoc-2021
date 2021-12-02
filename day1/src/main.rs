use std::fs;
use std::path::Path;

fn main() {
    let filepath = Path::new("./data/day-1-input.txt");

    let s = fs::read_to_string(filepath)
        .expect("Something went wrong reading the file");

    let v: Vec<u16> = s
        .split_terminator("\n")
        .map(|x| x.parse::<u16>().unwrap())
        .collect();

    let mut increase_count = 0;
    let mut current_sum: u16;
    let mut prev_sum: u16 = 0;

    let iter = v.windows(3);
    for items in iter {
        current_sum = items.iter().sum();
        
        increase_count = match prev_sum {
            ps if ps < current_sum => increase_count + 1,
            _ => increase_count
        };

        prev_sum = current_sum;
    }
    
    // the first iteration will always give you an extra increase because there was no previous sum
    // and prev_sum is set to 0.
    increase_count -= 1;
    println!("Increases: {}", increase_count);
}
