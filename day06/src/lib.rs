use itertools::Itertools;

pub fn process_part1(message: &str) -> usize {
    find_first_marker(message, 4)
}

pub fn process_part2(message: &str) -> usize {
    find_first_marker(message, 14)
}

fn find_first_marker(message: &str, marker_length: usize) -> usize {
    let vec: Vec<char> = message.chars().collect();
    let slice = &vec[..];
    match slice.windows(marker_length).position(are_all_unique) {
        Some(idx) => idx + marker_length,
        None => message.len(),
    }
}

fn are_all_unique(slice: &[char]) -> bool {
    slice.iter().all_unique()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_first_marker() {
        let message = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(7, find_first_marker(message, 4));

        let message = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(5, find_first_marker(message, 4));

        let message = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(6, find_first_marker(message, 4));

        let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(10, find_first_marker(message, 4));

        let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(11, find_first_marker(message, 4));

        let message = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(19, find_first_marker(message, 14));

        let message = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(23, find_first_marker(message, 14));

        let message = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(23, find_first_marker(message, 14));

        let message = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(29, find_first_marker(message, 14));

        let message = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(26, find_first_marker(message, 14));
    }
}
