pub fn process_part1(input: &str) -> String {
    input.to_uppercase()
}

pub fn process_part2(input: &str) -> String {
    input.to_uppercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_part1() {
        let input = "input";
        let result = process_part1(input);
        assert_eq!("INPUT", result);
    }

    #[test]
    fn test_process_part2() {
        let input = "input";
        let result = process_part1(input);
        assert_eq!("INPUT", result);
    }
}
