use day10::process_part1;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("./input/input.txt").unwrap();
  let output = process_part1(&input);
  println!("{}", output);  
}