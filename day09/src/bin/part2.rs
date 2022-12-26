use day09::process_part2;
use std::fs::read_to_string;

fn main() {
  let input = read_to_string("./input/input.txt").unwrap();
  let output = process_part2(&input);
  println!("{}", output);  
}