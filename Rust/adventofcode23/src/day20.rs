enum Module {
    Conjunction,
    FlipFlop,
    Broadcast,
}

impl Module  {
    fn get_default_pulse(&self) -> Pulse {
        match self {
            Module::Conjunction => Pulse::Low,
            Module::FlipFlop => Pulse::Low,
            Module::Broadcast => Pulse::High,
        }
    }
}

enum Pulse {
    High,
    Low,
}

struct State {
    name: String,
    module: Module,
    pulse: Pulse,
}

struct Wire {
    From: Module,
    To: Vec<State>,
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

pub fn day() -> String {
    let input = include_str!("../input/day20.txt");
    format!("Day 20\tPart 1: {}\tPart 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#;

    const INPUT2: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 32000000);
        assert_eq!(part1(INPUT2), 11687500);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 952408144115);
    }
}
