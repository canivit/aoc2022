pub fn process_part1(input: &str) -> String {
    let (mut piles, instructions) = parse_input(input);
    piles.perform_instructions(&instructions, MoveType::OneByOne);
    piles.top_crates()
}

pub fn process_part2(input: &str) -> String {
    let (mut piles, instructions) = parse_input(input);
    piles.perform_instructions(&instructions, MoveType::AllAtOnce);
    piles.top_crates()
}

#[derive(PartialEq, Debug)]
struct Instruction {
    count: usize,
    source: usize,
    dest: usize,
}

enum MoveType {
    OneByOne,
    AllAtOnce,
}

struct Piles(Vec<Vec<char>>);

impl Piles {
    fn perform_instructions(&mut self, instructions: &[Instruction], move_type: MoveType) {
        for instr in instructions {
            match move_type {
                MoveType::OneByOne => self.move_one_by_one(instr),
                MoveType::AllAtOnce => self.move_all_at_once(instr),
            }
        }
    }

    fn top_crates(&self) -> String {
        self.0.iter().map(|stack| stack.last().unwrap()).collect()
    }

    fn move_one_by_one(&mut self, instr: &Instruction) {
        for _ in 0..instr.count {
            let item = self.0[instr.source].pop().unwrap();
            self.0[instr.dest].push(item);
        }
    }

    fn move_all_at_once(&mut self, instr: &Instruction) {
        let mut temp: Vec<char> = Vec::new();
        for _ in 0..instr.count {
            let item = self.0[instr.source].pop().unwrap();
            temp.push(item);
        }

        for item in temp.into_iter().rev() {
            self.0[instr.dest].push(item);
        }
    }
}

fn parse_input(input: &str) -> (Piles, Vec<Instruction>) {
    let (stack_input, instr_input) = input.split_once("\n\n").unwrap();
    (parse_piles(stack_input), parse_instructions(instr_input))
}

fn parse_piles(input: &str) -> Piles {
    let (stack_lines, numbers) = input.rsplit_once("\n").unwrap();
    let num_of_stacks: usize = numbers.split_whitespace().last().unwrap().parse().unwrap();
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); num_of_stacks];

    for line in stack_lines.lines().rev() {
        let chars: Vec<char> = line.chars().collect();
        for (idx, chunk) in chars.chunks(4).into_iter().enumerate() {
            let maybe_crate = chunk[1];
            if !maybe_crate.is_whitespace() {
                stacks[idx].push(maybe_crate);
            }
        }
    }

    return Piles(stacks);
}

fn parse_instructions(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|line| {
            let nums: Vec<usize> = line
                .trim()
                .split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            Instruction {
                count: nums[0],
                source: nums[1] - 1,
                dest: nums[2] - 1,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perform_instructions_all_at_once() {
        let mut piles = Piles(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        let instructions = vec![
            Instruction {
                count: 1,
                source: 1,
                dest: 0,
            },
            Instruction {
                count: 3,
                source: 0,
                dest: 2,
            },
            Instruction {
                count: 2,
                source: 1,
                dest: 0,
            },
            Instruction {
                count: 1,
                source: 0,
                dest: 1,
            },
        ];

        piles.perform_instructions(&instructions, MoveType::AllAtOnce);
        assert_eq!("MCD", piles.top_crates())
    }

    #[test]
    fn test_perform_instructions_one_by_one() {
        let mut piles = Piles(vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        let instructions = vec![
            Instruction {
                count: 1,
                source: 1,
                dest: 0,
            },
            Instruction {
                count: 3,
                source: 0,
                dest: 2,
            },
            Instruction {
                count: 2,
                source: 1,
                dest: 0,
            },
            Instruction {
                count: 1,
                source: 0,
                dest: 1,
            },
        ];

        piles.perform_instructions(&instructions, MoveType::OneByOne);
        assert_eq!("CMZ", piles.top_crates())
    }

    #[test]
    fn test_parse_stacks() {
        let mut input = String::new();
        input.push_str("    [D]    \n");
        input.push_str("[N] [C]    \n");
        input.push_str("[Z] [M] [P]\n");
        input.push_str(" 1   2   3 ");

        let actual = parse_piles(&input);
        let expected = vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']];
        assert_eq!(actual.0, expected);
    }

    #[test]
    fn test_parse_instructions() {
        let input = "move 1 from 2 to 1
            move 3 from 1 to 3
            move 2 from 2 to 1
            move 1 from 1 to 2";

        let actual = parse_instructions(input);
        let expected = vec![
            Instruction {
                count: 1,
                source: 1,
                dest: 0,
            },
            Instruction {
                count: 3,
                source: 0,
                dest: 2,
            },
            Instruction {
                count: 2,
                source: 1,
                dest: 0,
            },
            Instruction {
                count: 1,
                source: 0,
                dest: 1,
            },
        ];
        assert_eq!(expected, actual);
    }
}
