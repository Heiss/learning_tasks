use std::str::FromStr;

type Id = usize;

#[derive(PartialEq, Debug)]
enum Block {
    Free,
    File(Id),
}

struct Disk {
    space: Vec<Block>,
}

impl Disk {
    fn reduction(mut self) -> ReducedDisk {
        let mut current_right_ptr = self.space.len() - 1;
        for current_left_ptr in 0..self.space.len() {
            if current_left_ptr >= current_right_ptr {
                break;
            }

            if self.space[current_left_ptr] != Block::Free {
                continue;
            }

            self.space.swap(current_left_ptr, current_right_ptr);
            while self.space[current_right_ptr] == Block::Free {
                current_right_ptr -= 1;
            }
        }
        ReducedDisk { disk: self }
    }
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .char_indices()
            .flat_map(|(i, c)| {
                let n: usize = c.to_digit(10).unwrap() as usize;
                (0..n)
                    .map(|_| {
                        if i % 2 == 0 {
                            Block::File(i / 2)
                        } else {
                            Block::Free
                        }
                    })
                    .collect::<Vec<Block>>()
            })
            .collect();
        Ok(Self { space: res })
    }
}

struct ReducedDisk {
    disk: Disk,
}

impl ReducedDisk {
    fn checksum(&self) -> usize {
        self.disk
            .space
            .iter()
            .enumerate()
            .map(|(i, n)| {
                if let Block::File(num) = *n {
                    i * num
                } else {
                    0
                }
            })
            .sum()
    }
}

fn part1(input: &str) -> usize {
    Disk::from_str(input).unwrap().reduction().checksum()
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
        assert_eq!(part2(INPUT), 258);
    }
}
