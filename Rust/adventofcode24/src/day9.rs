fn part1(input: &str) -> usize {
    0
}

fn part2(_input: &str) -> usize {
    0
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode24/day9.txt");
    format!("Day 9\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"2333133121414131402"#;
    #[test]
    fn day1() {
        assert_eq!(part1(INPUT), 1928);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 0);
    }
}
