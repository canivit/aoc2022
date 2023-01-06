pub fn process_part1(input: &str) -> isize {
    let instructions = parse_input(&input);
    let state = exec_all(&instructions);
    sum_signal_strength(20, 220, 40, &state.hist)
}

pub fn process_part2(input: &str) -> String {
    let instructions = parse_input(&input);
    let state = exec_all(&instructions);
    let pixels = lit_pixels(&state.hist[0..state.hist.len() - 1]);
    draw_pixels(&pixels)
}

enum Instr {
    Add(isize),
    Noop,
}

struct State {
    curr: isize,
    hist: Vec<isize>,
}

impl Instr {
    fn from_line(line: &str) -> Option<Self> {
        let tokens: Vec<&str> = line.trim().split_whitespace().collect();
        match tokens.as_slice() {
            ["addx", value] => {
                let value = value.parse::<isize>();
                if let Ok(val) = value {
                    Some(Instr::Add(val))
                } else {
                    None
                }
            }
            ["noop"] => Some(Instr::Noop),
            _ => None,
        }
    }

    fn exec(&self, mut state: State) -> State {
        match &self {
            Instr::Add(val) => {
                let next = state.curr + val;
                state.hist.push(state.curr);
                state.hist.push(next);
                state.curr = next;
                state
            }
            Instr::Noop => {
                state.hist.push(state.curr);
                state
            }
        }
    }
}

fn parse_input(input: &str) -> Vec<Instr> {
    input.lines().flat_map(Instr::from_line).collect()
}

fn exec_all(instructions: &[Instr]) -> State {
    let state = State {
        curr: 1,
        hist: vec![1],
    };

    instructions
        .iter()
        .fold(state, |acc, instr| instr.exec(acc))
}

fn sum_signal_strength(start: usize, end: usize, step: usize, hist: &[isize]) -> isize {
    (start - 1..end)
        .step_by(step)
        .map(|i| (i + 1) as isize * hist.get(i).unwrap_or(&0))
        .sum()
}

fn lit_pixels(hist: &[isize]) -> Vec<bool> {
    hist.iter()
        .enumerate()
        .map(|pair| {
            let crt = (pair.0 % 40) as isize;
            let sprite = pair.1;
            crt >= sprite - 1 && crt <= sprite + 1
        })
        .collect()
}

fn draw_pixels(pixels: &[bool]) -> String {
    pixels
        .chunks(40)
        .map(|row| draw_row(row))
        .collect::<Vec<String>>()
        .join("\n")
}

fn draw_row(row: &[bool]) -> String {
    row.iter()
        .map(|pixel| if *pixel { '#' } else { '.' })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exec_all() {
        let instructions = vec![Instr::Noop, Instr::Add(3), Instr::Add(-5)];
        let hist = exec_all(&instructions).hist;
        assert_eq!([1, 1, 1, 4, 4, -1], hist.as_slice());
    }

    #[test]
    fn test_process_part1() {
        let input = "addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop";

        assert_eq!(13140, process_part1(input));
    }

    #[test]
    fn test_process_part2() {
        let input = "addx 15
            addx -11
            addx 6
            addx -3
            addx 5
            addx -1
            addx -8
            addx 13
            addx 4
            noop
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx 5
            addx -1
            addx -35
            addx 1
            addx 24
            addx -19
            addx 1
            addx 16
            addx -11
            noop
            noop
            addx 21
            addx -15
            noop
            noop
            addx -3
            addx 9
            addx 1
            addx -3
            addx 8
            addx 1
            addx 5
            noop
            noop
            noop
            noop
            noop
            addx -36
            noop
            addx 1
            addx 7
            noop
            noop
            noop
            addx 2
            addx 6
            noop
            noop
            noop
            noop
            noop
            addx 1
            noop
            noop
            addx 7
            addx 1
            noop
            addx -13
            addx 13
            addx 7
            noop
            addx 1
            addx -33
            noop
            noop
            noop
            addx 2
            noop
            noop
            noop
            addx 8
            noop
            addx -1
            addx 2
            addx 1
            noop
            addx 17
            addx -9
            addx 1
            addx 1
            addx -3
            addx 11
            noop
            noop
            addx 1
            noop
            addx 1
            noop
            noop
            addx -13
            addx -19
            addx 1
            addx 3
            addx 26
            addx -30
            addx 12
            addx -1
            addx 3
            addx 1
            noop
            noop
            noop
            addx -9
            addx 18
            addx 1
            addx 2
            noop
            noop
            addx 9
            noop
            noop
            noop
            addx -1
            addx 2
            addx -37
            addx 1
            addx 3
            noop
            addx 15
            addx -21
            addx 22
            addx -6
            addx 1
            noop
            addx 2
            addx 1
            noop
            addx -10
            noop
            noop
            addx 20
            addx 1
            addx 2
            addx 2
            addx -6
            addx -11
            noop
            noop
            noop";

        let output = vec![
            "##..##..##..##..##..##..##..##..##..##..",
            "###...###...###...###...###...###...###.",
            "####....####....####....####....####....",
            "#####.....#####.....#####.....#####.....",
            "######......######......######......####",
            "#######.......#######.......#######.....",
        ]
        .join("\n");

        assert_eq!(output, process_part2(input))
    }
}
