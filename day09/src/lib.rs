use std::collections::HashSet;

pub fn process_part1(input: &str) -> usize {
    process_help(input, 2)
}

pub fn process_part2(input: &str) -> usize {
    process_help(input, 10)
}

fn process_help(input: &str, rope_length: usize) -> usize {
    let motions = parse_motions(input);
    let rope = Rope::new(rope_length);
    let mut ropes = rope.perform_motions(motions);
    ropes.push(rope);
    num_of_distinct_tail_posns(ropes)
}

fn parse_motions(input: &str) -> Vec<Motion> {
    input
        .lines()
        .filter_map(|line| parse_motion(line.trim()))
        .collect()
}

fn parse_motion(line: &str) -> Option<Motion> {
    let maybe_pair = line.split_once(" ");
    let maybe_dir = maybe_pair.and_then(|pair| pair.0.chars().nth(0));
    let maybe_count = maybe_pair.and_then(|pair| pair.1.parse::<usize>().ok());

    match (maybe_dir, maybe_count) {
        (Some('U'), Some(count)) => Some(Motion {
            direction: Direction::Up,
            count: count,
        }),
        (Some('D'), Some(count)) => Some(Motion {
            direction: Direction::Down,
            count: count,
        }),
        (Some('L'), Some(count)) => Some(Motion {
            direction: Direction::Left,
            count: count,
        }),
        (Some('R'), Some(count)) => Some(Motion {
            direction: Direction::Right,
            count: count,
        }),
        _ => None,
    }
}

fn num_of_distinct_tail_posns(ropes: Vec<Rope>) -> usize {
    HashSet::<_>::from_iter(ropes.iter().filter_map(|rope| rope.knots.last())).len()
}

struct Rope {
    knots: Vec<Posn>,
}

#[derive(Copy, Clone, Hash, Eq, PartialEq)]
struct Posn {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Copy, Clone)]
struct Motion {
    direction: Direction,
    count: usize,
}

impl Rope {
    fn new(length: usize) -> Self {
        let length = if length <= 0 { 1 } else { length };
        Rope {
            knots: vec![Posn { x: 0, y: 0 }; length],
        }
    }

    fn perform_motions(&self, motions: Vec<Motion>) -> Vec<Rope> {
        let mut ropes: Vec<Rope> = Vec::new();
        let mut rope = self;
        for motion in motions {
            for _ in 0..motion.count {
                let next_rope = rope.move_one_step(motion.direction);
                ropes.push(next_rope);
                rope = &ropes[ropes.len() - 1];
            }
        }

        ropes
    }

    fn move_one_step(&self, direction: Direction) -> Self {
        let mut head = self.knots[0].move_one_step(direction);
        let mut knots = vec![head];
        for next in self.knots[1..].iter() {
            head = next.move_relativete_to_head(head);
            knots.push(head);
        }
        Rope { knots }
    }
}

impl Posn {
    fn move_one_step(self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Down => Self {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Left => Self {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Self {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn move_relativete_to_head(self, head: Posn) -> Posn {
        let x_diff = head.x - self.x;
        let y_diff = head.y - self.y;
        if x_diff.abs() > 1 || y_diff.abs() > 1 {
            Posn {
                x: self.x + x_diff.signum(),
                y: self.y + y_diff.signum(),
            }
        } else {
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "
            R 4
            U 4
            L 3
            D 1
            R 4
            D 1
            L 5
            R 2";
        let result = process_part1(input);
        assert_eq!(13, result);
    }

    #[test]
    fn test_move_one_step() {
        let rope = Rope::new(10);
        let ropes = rope.perform_motions(vec![
            Motion {
                direction: Direction::Right,
                count: 4,
            },
            Motion {
                direction: Direction::Up,
                count: 4,
            },
        ]);

        let last_rope = ropes.last().unwrap();

        let head = &last_rope.knots[0];
        assert_eq!(4, head.x);
        assert_eq!(-4, head.y);

        let knot = &last_rope.knots[1];
        assert_eq!(4, knot.x);
        assert_eq!(-3, knot.y);

        let knot = &last_rope.knots[2];
        assert_eq!(4, knot.x);
        assert_eq!(-2, knot.y);

        let knot = &last_rope.knots[3];
        assert_eq!(3, knot.x);
        assert_eq!(-2, knot.y);

        let knot = &last_rope.knots[4];
        assert_eq!(2, knot.x);
        assert_eq!(-2, knot.y);

        let knot = &last_rope.knots[5];
        assert_eq!(1, knot.x);
        assert_eq!(-1, knot.y);
    }
}
