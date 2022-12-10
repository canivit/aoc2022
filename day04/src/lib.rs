pub fn process_part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| line_to_range_pair(line.trim()))
        .filter(|pair| pair.0.does_one_fully_contain_other(&pair.1))
        .count()
}

pub fn process_part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| line_to_range_pair(line.trim()))
        .filter(|pair| pair.0.does_overlap(&pair.1))
        .count()
}

#[derive(PartialEq, Debug)]
struct Range {
    start: u32,
    end: u32,
}

impl Range {
    fn does_fully_contain(&self, other: &Range) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn does_one_fully_contain_other(&self, other: &Range) -> bool {
        self.does_fully_contain(other) || other.does_fully_contain(self)
    }

    fn does_overlap(&self, other: &Range) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (other.start >= self.start && other.start <= self.end)
    }
}

fn line_to_range_pair(line: &str) -> (Range, Range) {
    let (part1, part2) = line.split_once(",").unwrap();
    (line_part_to_range(part1), line_part_to_range(part2))
}

fn line_part_to_range(line_part: &str) -> Range {
    let (start, end) = line_part.split_once("-").unwrap();
    Range {
        start: start.parse().unwrap(),
        end: end.parse().unwrap(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8";
        let result = process_part1(input);
        assert_eq!(2, result);
    }

    #[test]
    fn test_process_part2() {
        let input = "2-4,6-8
            2-3,4-5
            5-7,7-9
            2-8,3-7
            6-6,4-6
            2-6,4-8";
        let result = process_part2(input);
        assert_eq!(4, result);
    }

    #[test]
    fn test_does_overlap() {
        let range1 = Range { start: 3, end: 7 };
        let range2 = Range { start: 5, end: 8 };
        let range3 = Range { start: 0, end: 4 };
        let range4 = Range { start: 4, end: 6 };
        assert!(range1.does_overlap(&range2));
        assert!(range2.does_overlap(&range1));
        assert!(range1.does_overlap(&range4));
        assert!(range4.does_overlap(&range1));
        assert!(!range2.does_overlap(&range3));
        assert!(!range3.does_overlap(&range2));
    }

    #[test]
    fn test_does_one_fully_contain_other() {
        let range1 = Range { start: 1, end: 7 };
        let range2 = Range { start: 3, end: 6 };
        let range3 = Range { start: 3, end: 8 };
        assert!(range1.does_one_fully_contain_other(&range2));
        assert!(range2.does_one_fully_contain_other(&range1));
        assert!(!range1.does_one_fully_contain_other(&range3));
        assert!(!range3.does_one_fully_contain_other(&range1));
    }

    #[test]
    fn test_line_to_range_pair() {
        let (range1, range2) = line_to_range_pair("2-3,4-5");
        assert_eq!(Range { start: 2, end: 3 }, range1);
        assert_eq!(Range { start: 4, end: 5 }, range2);
    }

    #[test]
    fn test_line_part_to_range() {
        assert_eq!(Range { start: 2, end: 6 }, line_part_to_range("2-6"));
    }
}
