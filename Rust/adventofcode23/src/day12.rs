#![allow(dead_code)]

use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::mem::MaybeUninit;
use std::str::FromStr;
use std::sync::Mutex;
use std::sync::Once;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum SpringState {
    // #
    Damaged,
    // ?
    Unknown,
    // .
    Okay,
}

impl FromStr for SpringState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "#" => Ok(Self::Damaged),
            "?" => Ok(Self::Unknown),
            "." => Ok(Self::Okay),
            _ => panic!("invalid spring state: {}", s),
        }
    }
}

impl SpringState {
    fn get_possibilities(&self) -> Vec<SpringState> {
        match self {
            Self::Damaged => vec![Self::Damaged],
            Self::Unknown => vec![Self::Damaged, Self::Okay],
            Self::Okay => vec![Self::Okay],
        }
    }
}

impl Debug for SpringState {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Damaged => write!(f, "#"),
            Self::Unknown => write!(f, "?"),
            Self::Okay => write!(f, "."),
        }
    }
}

type Solution = (Vec<SpringState>, Vec<usize>, usize);

fn get_solutions() -> &'static Mutex<HashMap<Solution, usize>> {
    static mut SOLUTIONS: MaybeUninit<Mutex<HashMap<Solution, usize>>> = MaybeUninit::uninit();
    static INIT: Once = Once::new();

    INIT.call_once(|| unsafe {
        SOLUTIONS.as_mut_ptr().replace(Mutex::new(HashMap::new()));
    });

    unsafe { &*SOLUTIONS.as_ptr() }
}

struct SpringGroup {
    pattern: Vec<SpringState>,
    sizes: Vec<usize>,
}

impl Debug for SpringGroup {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let sizes = self
            .sizes
            .iter()
            .map(|s| format!("{s:?}"))
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{:?} {:?}", self.pattern, sizes)
    }
}

fn num_solutions(
    s: Vec<SpringState>,
    sizes: Vec<usize>,
    num_done_in_group: Option<usize>,
) -> usize {
    let sol = get_solutions();
    let num_done_in_group = num_done_in_group.unwrap_or(0);
    if let Some(num_sols) = sol
        .lock()
        .unwrap()
        .get(&(s.clone(), sizes.clone(), num_done_in_group))
    {
        return *num_sols;
    }

    if s.is_empty() {
        return (sizes.len() == 0 && num_done_in_group == 0) as usize;
    }

    let mut num_sols = 0;

    let possibilities = s[0].get_possibilities();
    for possibility in possibilities {
        let s_next = s.clone()[1..].to_vec();
        match possibility {
            SpringState::Damaged => {
                num_sols += num_solutions(s_next, sizes.clone(), Some(num_done_in_group + 1));
            }
            _ => {
                if num_done_in_group > 0 {
                    if let Some(size) = sizes.first() {
                        if *size == num_done_in_group {
                            num_sols += num_solutions(s_next, sizes[1..].to_vec(), None);
                        }
                    }
                } else {
                    num_sols += num_solutions(s_next, sizes.clone(), None);
                }
            }
        }
    }

    sol.lock()
        .unwrap()
        .insert((s, sizes, num_done_in_group), num_sols);

    num_sols
}

impl SpringGroup {
    fn get_solutions_count(&self) -> usize {
        num_solutions(self.pattern.clone(), self.sizes.clone(), None)
    }
}

struct Springs {
    groups: Vec<SpringGroup>,
}

impl Springs {
    fn get_solutions_count(&mut self) -> usize {
        self.groups
            .iter_mut()
            .map(|g| {
                g.pattern.push(SpringState::Okay);
                g.get_solutions_count()
            })
            .sum()
    }

    fn unfold(&mut self, n: usize) {
        for group in self.groups.iter_mut() {
            let old_pattern = group.pattern.clone();
            let old_sizes = group.sizes.clone();
            for _ in 1..n {
                group.pattern.push(SpringState::Unknown);
                group.pattern.extend(old_pattern.clone());
                group.sizes.extend(old_sizes.clone());
            }
        }
    }
}

impl Debug for Springs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let groups = self
            .groups
            .iter()
            .map(|g| format!("{g:?}"))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "{}", groups)
    }
}

impl FromStr for Springs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups = Vec::new();
        for line in s.lines() {
            let mut parts = line.split_whitespace();
            let pattern: Vec<_> = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| c.to_string().parse().unwrap())
                .collect();

            let sizes = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect();
            groups.push(SpringGroup { pattern, sizes });
        }

        Ok(Self { groups })
    }
}

fn part1(input: &str) -> usize {
    let mut springs = input.parse::<Springs>().unwrap();
    springs.get_solutions_count()
}

fn part2(input: &str) -> usize {
    let mut springs = input.parse::<Springs>().unwrap();
    springs.unfold(5);
    springs.get_solutions_count()
}

pub fn day() -> String {
    let input = include_str!("../input/day12.txt");
    format!("Day 12\tPart 1: {}\tPart 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 21);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 525152);
    }
}
