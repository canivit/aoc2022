use day08::process_part1;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("./input/input.txt").unwrap();
    let result = process_part1(&input);
    match result {
        Ok(output) => println!("{}", output),
        Err(_) => println!("Invalid input"),
    }
}
