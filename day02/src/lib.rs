pub fn process_part1(input: &str) -> u32 {
    input.lines().map(|line| line_score_p1(line.trim())).sum()
}

pub fn process_part2(input: &str) -> u32 {
    input.lines().map(|line| line_score_p2(line.trim())).sum()
}

#[derive(Clone, Copy)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Clone, Copy)]
enum Outcome {
    Win,
    Draw,
    Lose,
}

const ROCK_SCORE: u32 = 1;
const PAPER_SCORE: u32 = 2;
const SCISSORS_SCORE: u32 = 3;

const WIN_SCORE: u32 = 6;
const DRAW_SCORE: u32 = 3;
const LOSE_SCORE: u32 = 0;

const LEFT_ROCK_CHAR: char = 'A';
const LEFT_PAPER_CHAR: char = 'B';
const LEFT_SCISSORS_CHAR: char = 'C';

const RIGHT_ROCK_CHAR: char = 'X';
const RIGHT_PAPER_CHAR: char = 'Y';
const RIGHT_SCISSORS_CHAR: char = 'Z';

const LOSE_CHAR: char = 'X';
const DRAW_CHAR: char = 'Y';
const WIN_CHAR: char = 'Z';

fn line_to_char_pair(line: &str) -> (char, char) {
    let left_char = line.split(" ").nth(0).unwrap().chars().nth(0).unwrap();
    let right_char = line.split(" ").nth(1).unwrap().chars().nth(0).unwrap();
    (left_char, right_char)
}

fn line_score_p1(line: &str) -> u32 {
    let (left_char, right_char) = line_to_char_pair(line);
    let left_shape = left_char_to_shape(left_char);
    let right_shape = right_char_to_shape(right_char);
    round_score(left_shape, right_shape)
}

fn line_score_p2(line: &str) -> u32 {
    let (left_char, right_char) = line_to_char_pair(line);
    let left_shape = left_char_to_shape(left_char);
    let outcome = char_to_outcome(right_char);
    let right_shape = find_right_shape(left_shape, outcome);
    shape_score(right_shape) + outcome_score(outcome)
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

fn char_to_outcome(c: char) -> Outcome {
    match c {
        LOSE_CHAR => Outcome::Lose,
        DRAW_CHAR => Outcome::Draw,
        WIN_CHAR => Outcome::Win,
        _ => panic!("Invalid outcome character"),
    }
}

fn round_score(left_shape: Shape, right_shape: Shape) -> u32 {
    shape_score(right_shape) + outcome_score(outcome_of_round(left_shape, right_shape))
}

fn shape_score(shape: Shape) -> u32 {
    match shape {
        Shape::Rock => ROCK_SCORE,
        Shape::Paper => PAPER_SCORE,
        Shape::Scissors => SCISSORS_SCORE,
    }
}

fn outcome_score(outcome: Outcome) -> u32 {
    match outcome {
        Outcome::Win => WIN_SCORE,
        Outcome::Draw => DRAW_SCORE,
        Outcome::Lose => LOSE_SCORE,
    }
}

fn outcome_of_round(left_shape: Shape, right_shape: Shape) -> Outcome {
    match (left_shape, right_shape) {
        (Shape::Rock, Shape::Rock) => Outcome::Draw,
        (Shape::Rock, Shape::Paper) => Outcome::Win,
        (Shape::Rock, Shape::Scissors) => Outcome::Lose,

        (Shape::Paper, Shape::Rock) => Outcome::Lose,
        (Shape::Paper, Shape::Paper) => Outcome::Draw,
        (Shape::Paper, Shape::Scissors) => Outcome::Win,

        (Shape::Scissors, Shape::Rock) => Outcome::Win,
        (Shape::Scissors, Shape::Paper) => Outcome::Lose,
        (Shape::Scissors, Shape::Scissors) => Outcome::Draw,
    }
}

fn find_right_shape(left_shape: Shape, outcome: Outcome) -> Shape {
    match (left_shape, outcome) {
        (Shape::Rock, Outcome::Win) => Shape::Paper,
        (Shape::Rock, Outcome::Draw) => Shape::Rock,
        (Shape::Rock, Outcome::Lose) => Shape::Scissors,
        (Shape::Paper, Outcome::Win) => Shape::Scissors,
        (Shape::Paper, Outcome::Draw) => Shape::Paper,
        (Shape::Paper, Outcome::Lose) => Shape::Rock,
        (Shape::Scissors, Outcome::Win) => Shape::Rock,
        (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
        (Shape::Scissors, Outcome::Lose) => Shape::Paper,
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
        let input = "A Y
            B X
            C Z";

        let result = process_part2(input);
        assert_eq!(12, result);
    }
}
