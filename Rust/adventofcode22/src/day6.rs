use itertools::Itertools;

struct CommunicationDevice<'a> {
    text: &'a str,
}

impl<'a> CommunicationDevice<'a> {
    fn new(text: &'a str) -> CommunicationDevice<'a> {
        CommunicationDevice { text }
    }

    fn get_position_of_marker(&self, marker_size: usize) -> Option<usize> {
        if self.text.len() < 4 {
            return None;
        }

        self.text
            .chars()
            .collect::<Vec<char>>()
            .windows(marker_size)
            .position(|window| window.iter().counts().values().all(|&count| count == 1))
            .and_then(|index| Some(index + marker_size))
    }
}

fn day6_part1(text: &str) -> usize {
    CommunicationDevice::new(text)
        .get_position_of_marker(4)
        .unwrap_or(0)
}

fn day6_part2(text: &str) -> usize {
    CommunicationDevice::new(text)
        .get_position_of_marker(14)
        .unwrap_or(0)
}

pub fn day6(text: &str) {
    println!("day6 part 1: {}", day6_part1(text));
    println!("day6 part 2: {}", day6_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day6_part1, day6_part2};

    const TEST_INPUT: [(&str, usize, usize); 5] = [
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 7, 19),
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", 5, 23),
        ("nppdvjthqldpwncqszvftbrmjlhg", 6, 23),
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 10, 29),
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 11, 26),
    ];

    #[test]
    fn fixed_tests1() {
        for (input, expected, _) in TEST_INPUT.iter() {
            assert_eq!(day6_part1(input), *expected);
        }
    }

    #[test]
    fn fixed_tests2() {
        for (input, _, expected) in TEST_INPUT.iter() {
            assert_eq!(day6_part2(input), *expected);
        }
    }
}
