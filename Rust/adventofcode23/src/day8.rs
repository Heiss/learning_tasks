use regex::Regex;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::Mutex;

#[derive(Debug)]
struct Node {
    name: String,
    left: Option<Rc<Mutex<Node>>>,
    right: Option<Rc<Mutex<Node>>>,
}

impl Node {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Left,
    Right,
}

#[derive(Debug)]
struct Map {
    instructions: Vec<Instruction>,
    map: HashMap<String, Rc<Mutex<Node>>>,
}

impl FromStr for Map {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parse_map = HashMap::new();

        let mut lines = s.splitn(2, "\n\n");
        let instructions = lines.next().unwrap();

        let re = Regex::new(r"^(\w{3}) = \((\w{3}), (\w{3})\)$").unwrap();

        let mut results = vec![];
        for line in lines.next().unwrap().lines() {
            for (_, [root, left, right]) in re.captures_iter(line).map(|c| c.extract()) {
                results.push((root, left, right));
            }
        }

        for (root, left, right) in results {
            let name = root.to_string();
            let node = parse_map
                .entry(name)
                .or_insert(Rc::new(Mutex::new(Node::new(root))))
                .clone();

            node.lock()
                .map(|mut lock| {
                    let mut nodes = [left, right]
                        .map(|n| {
                            parse_map
                                .entry(n.to_string())
                                .or_insert(Rc::new(Mutex::new(Node::new(n))))
                                .clone()
                        })
                        .map(Some)
                        .into_iter()
                        .collect::<Vec<_>>();

                    lock.right = nodes.pop().unwrap();
                    lock.left = nodes.pop().unwrap();
                })
                .unwrap();
        }

        Ok(Self {
            instructions: instructions
                .chars()
                .map(|c| match c {
                    'L' => Instruction::Left,
                    'R' => Instruction::Right,
                    _ => panic!("Invalid instruction"),
                })
                .collect(),
            map: parse_map,
        })
    }
}

impl Map {
    fn count_steps_with_instructions(&self) -> usize {
        let mut steps = 0;
        let mut current = self.map.get("AAA").expect("AAA must be there").clone();
        for instruction in self.instructions.iter().cycle() {
            current = {
                let current_lock = current.lock().unwrap();
                let next = match instruction {
                    Instruction::Left => current_lock
                        .left
                        .as_ref()
                        .expect("Left must be there")
                        .clone(),
                    Instruction::Right => current_lock
                        .right
                        .as_ref()
                        .expect("Right must be there")
                        .clone(),
                };
                if current_lock.name == "ZZZ" {
                    return steps;
                }
                steps += 1;
                next
            };
        }
        0
    }

    fn count_steps_as_ghost(&self) -> usize {
        let current: Vec<_> = self
            .map
            .iter()
            .filter_map(|(k, v)| {
                if k.chars().collect::<Vec<char>>().pop().unwrap() == 'A' {
                    Some(v.clone())
                } else {
                    None
                }
            })
            .collect();
        let mut loops = Vec::new();

        for node in current.into_iter() {
            let mut steps = 0;
            let mut current = node.clone();

            for instruction in self.instructions.iter().cycle() {
                current = {
                    let lock = current.lock().unwrap();

                    if lock.name.chars().collect::<Vec<_>>().pop().unwrap() == 'Z' {
                        loops.push(steps);
                        break;
                    }

                    match instruction {
                        Instruction::Left => {
                            lock.left.as_ref().expect("Left must be there").clone()
                        }
                        Instruction::Right => {
                            lock.right.as_ref().expect("Right must be there").clone()
                        }
                    }
                };
                steps += 1;
            }
        }

        lcm_vec(loops)
    }
}

fn lcm_vec(vec: Vec<usize>) -> usize {
    vec.into_iter().fold(1, |acc, x| lcm(acc, x))
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn part1(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();
    map.count_steps_with_instructions()
}

fn part2(input: &str) -> usize {
    let map = Map::from_str(input).unwrap();
    map.count_steps_as_ghost()
}

pub fn day() {
    let input = include_str!("../input/day8.txt");
    print!("Day 8\t");
    print!("Part 1: {}\t", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                r#"RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"#
            ),
            2
        );
        assert_eq!(
            part1(
                r#"LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"#
            ),
            6
        );
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                r#"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"#
            ),
            6
        );
    }
}
