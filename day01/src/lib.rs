pub fn process_part1(input: &str) -> String {
    input
        .split("\n\n")
        .map(sum_group)
        .max()
        .unwrap()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input
        .split("\n\n")
        .map(sum_group)
        .fold([0, 0, 0], update_top_three)
        .iter()
        .sum::<u32>()
        .to_string()
}

fn sum_group(group: &str) -> u32 {
    group
        .lines()
        .map(|calorie| calorie.trim().parse::<u32>().unwrap())
        .sum::<u32>()
}

fn update_top_three(top_three: [u32; 3], next: u32) -> [u32; 3] {
    let mut vec = top_three.to_vec();
    vec.push(next);
    vec.sort_by(|a, b| b.cmp(a));
    [vec[0], vec[1], vec[2]]
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
    fn test_process_part2() {
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

        let result = process_part2(input);
        assert_eq!("45000", result);
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
    fn test_update_top_three() {
        let top_three = [0, 0, 0];
        let new_top_three = [2, 0, 0];
        assert_eq!(new_top_three, update_top_three(top_three, 2));

        let top_three = [8, 5, 3];
        let new_top_three = [9, 8, 5];
        assert_eq!(new_top_three, update_top_three(top_three, 9));

        let top_three = [8, 5, 3];
        assert_eq!(top_three, update_top_three(top_three, 2));
    }
    
}
