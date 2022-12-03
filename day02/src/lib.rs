pub fn process_part1(input: &str) -> u32 {
    input.lines().map(|line| line_score(line.trim())).sum()
}

pub fn process_part2(input: &str) -> String {
    input.to_uppercase()
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const LOSE_SCORE: u32 = 0;
const DRAW_SCORE: u32 = 3;
const WIN_SCORE: u32 = 6;

const LEFT_ROCK_CHAR: char = 'A';
const LEFT_PAPER_CHAR: char = 'B';
const LEFT_SCISSORS_CHAR: char = 'C';

const RIGHT_ROCK_CHAR: char = 'X';
const RIGHT_PAPER_CHAR: char = 'Y';
const RIGHT_SCISSORS_CHAR: char = 'Z';

fn line_score(line: &str) -> u32 {
    let left_char = line.split(" ").nth(0).unwrap().chars().nth(0).unwrap();
    let left_shape = left_char_to_shape(left_char);

    let right_char = line.split(" ").nth(1).unwrap().chars().nth(0).unwrap();
    let right_shape = right_char_to_shape(right_char);

    round_score(left_shape, right_shape)
}

fn left_char_to_shape(left_char: char) -> Shape {
    match left_char {
        LEFT_ROCK_CHAR => Shape::Rock,
        LEFT_PAPER_CHAR => Shape::Paper,
        LEFT_SCISSORS_CHAR => Shape::Scissors,
        _ => panic!("Invalid shape character"),
    }
}

fn right_char_to_shape(right_char: char) -> Shape {
    match right_char {
        RIGHT_ROCK_CHAR => Shape::Rock,
        RIGHT_PAPER_CHAR => Shape::Paper,
        RIGHT_SCISSORS_CHAR => Shape::Scissors,
        _ => panic!("Invalid shape character"),
    }
}

fn round_score(left_shape: Shape, right_shape: Shape) -> u32 {
    shape_score(right_shape) + outcome_score(left_shape, right_shape)
}

fn shape_score(shape: Shape) -> u32 {
    match shape {
        Shape::Rock => ROCK_SCORE,
        Shape::Paper => PAPER_SCORE,
        Shape::Scissors => SCISSORS_SCORE,
    }
}

fn outcome_score(left_shape: Shape, right_shape: Shape) -> u32 {
    match (left_shape, right_shape) {
        (Shape::Rock, Shape::Rock) => DRAW_SCORE,
        (Shape::Rock, Shape::Paper) => WIN_SCORE,
        (Shape::Rock, Shape::Scissors) => LOSE_SCORE,

        (Shape::Paper, Shape::Rock) => LOSE_SCORE,
        (Shape::Paper, Shape::Paper) => DRAW_SCORE,
        (Shape::Paper, Shape::Scissors) => WIN_SCORE,

        (Shape::Scissors, Shape::Rock) => WIN_SCORE,
        (Shape::Scissors, Shape::Paper) => LOSE_SCORE,
        (Shape::Scissors, Shape::Scissors) => DRAW_SCORE,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "A Y
            B X
            C Z";

        let result = process_part1(input);
        assert_eq!(15, result);
    }

    #[test]
    fn test_process_part2() {
        let input = "input";
        let result = process_part2(input);
        assert_eq!("INPUT", result);
    }
}
