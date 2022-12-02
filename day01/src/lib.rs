pub fn process_part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(sum_group)
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input.to_uppercase()
}

fn sum_group(group: &str) -> u32 {
    group
        .lines()
        .map(|calorie| calorie.trim().parse::<u32>().unwrap())
        .sum::<u32>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000";

        let result = process_part1(input);
        assert_eq!("24000", result);
    }

    #[test]
    fn test_sum_group() {
        let group = "125";
        let result = sum_group(group);
        assert_eq!(125, result);

        let group = "125\n65";
        let result = sum_group(group);
        assert_eq!(190, result);

        let group = "125\n65\n140\n82\n105";
        let result = sum_group(group);
        assert_eq!(517, result);
    }

    #[test]
    fn test_process_part2() {
        let input = "input";
        let result = process_part2(input);
        assert_eq!("INPUT", result);
    }
}
