use std::collections::HashSet;

pub fn process_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| priority_of_word(line.trim()))
        .sum()
}

pub fn process_part2(input: &str) -> u32 {
    input
        .lines()
        .map(&str::trim)
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(priority_of_group)
        .sum()
}

fn priority_of_group(group: &[&str]) -> u32 {
    let common_chars = find_common_chars(group[0], &find_common_chars(group[1], group[2]));
    common_chars.chars().map(priority_of_char).sum()
}

fn priority_of_word(word: &str) -> u32 {
    let (first, second) = split_into_half(word);
    let common_chars = find_common_chars(first, second);
    common_chars.chars().map(priority_of_char).sum()
}

fn priority_of_char(c: char) -> u32 {
    (if c.is_lowercase() {
        (c as u8) - 96
    } else {
        (c as u8) - 38
    })
    .into()
}

fn split_into_half(word: &str) -> (&str, &str) {
    let mid = word.len() / 2;
    (&word[..mid], &word[mid..])
}

fn find_common_chars(s1: &str, s2: &str) -> String {
    let set1: HashSet<char> = HashSet::from_iter(s1.chars());
    let set2: HashSet<char> = HashSet::from_iter(s2.chars());
    set1.into_iter().filter(|c| set2.contains(c)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = process_part1(input);
        assert_eq!(157, result);
    }

    #[test]
    fn test_process_part2() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
            jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
            PmmdzqPrVvPwwTWBwg
            wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
            ttgJtRGJQctTZtZT
            CrZsJsPPZsGzwwsLwLmpwMDw";

        let result = process_part2(input);
        assert_eq!(70, result);
    }

    #[test]
    fn test_priority_of_group() {
        let group = &[
            "vJrwpWtwJgWrhcsFMMfFFhFp",
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
        ];
        assert_eq!(18, priority_of_group(group));

        let group = &[
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ];
        assert_eq!(52, priority_of_group(group));
    }

    #[test]
    fn test_priority_of_word() {
        assert_eq!(16, priority_of_word("vJrwpWtwJgWrhcsFMMfFFhFp"));
        assert_eq!(38, priority_of_word("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"));
        assert_eq!(42, priority_of_word("PmmdzqPrVvPwwTWBwg"));
        assert_eq!(22, priority_of_word("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"));
        assert_eq!(20, priority_of_word("ttgJtRGJQctTZtZT"));
        assert_eq!(19, priority_of_word("CrZsJsPPZsGzwwsLwLmpwMDw"));
    }

    #[test]
    fn test_priority_of_char() {
        assert_eq!(1, priority_of_char('a'));
        assert_eq!(2, priority_of_char('b'));
        assert_eq!(3, priority_of_char('c'));
        assert_eq!(26, priority_of_char('z'));
        assert_eq!(27, priority_of_char('A'));
        assert_eq!(28, priority_of_char('B'));
        assert_eq!(29, priority_of_char('C'));
        assert_eq!(52, priority_of_char('Z'));
    }

    #[test]
    fn test_split_into_half() {
        let (first, second) = split_into_half("PmmdzqPrVvPwwTWBwg");
        assert_eq!("PmmdzqPrV", first);
        assert_eq!("vPwwTWBwg", second);
    }

    #[test]
    fn test_find_common_chars() {
        let common_chars = find_common_chars("vJrwpWtwJgWr", "hcsFMMfFFhFp");
        assert_eq!("p", common_chars);

        let common_chars = find_common_chars("jqHRNqRjqzjGDLGL", "rsFMfFZSrLrFZsSL");
        assert_eq!("L", common_chars);
    }
}
