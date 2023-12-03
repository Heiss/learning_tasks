fn part1(input: &str) -> usize {
    todo!("Implement part 1");
}

fn part2(input: &str) -> usize {
    todo!("Implement part 2");
}

pub fn day() {
    let input = include_str!("../input/day4.txt");
    print!("Day 4\t");
    print!("Part 1: {}\t", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#""#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 4361);
    }

    #[test]
    fn test_test2() {
        assert_eq!(part2(INPUT), 467835);
    }
}
