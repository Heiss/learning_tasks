use std::str::FromStr;

type Id = usize;
type Length = usize;

#[derive(PartialEq, Debug)]
enum Block {
    Free(Length),
    File(FileBlock),
}

#[derive(Debug, PartialEq)]
struct FileBlock {
    id: Id,
    length: Length,
}

struct Disk {
    space: Vec<Block>,
}

impl Disk {
    fn reduction(mut self) -> ReducedDisk {
        let new_space = self
            .space
            .iter()
            .flat_map(|f| match f {
                Block::Free(n) => (0..*n).map(|_| Block::Free(1)).collect::<Vec<Block>>(),
                Block::File(FileBlock { id, length }) => (0..*length)
                    .map(|_| Block::File(FileBlock { id: *id, length: 1 }))
                    .collect::<Vec<Block>>(),
            })
            .collect();
        self.space = new_space;

        let mut current_right_ptr = self.space.len() - 1;
        for current_left_ptr in 0..self.space.len() {
            if current_left_ptr >= current_right_ptr {
                break;
            }

            if let Block::File(_) = self.space[current_left_ptr] {
                continue;
            }

            self.space.swap(current_left_ptr, current_right_ptr);
            while let Block::Free(_) = self.space[current_right_ptr] {
                current_right_ptr -= 1;
            }
        }
        ReducedDisk { disk: self }
    }

    fn defragmantation(mut self) -> ReducedDisk {
        let mut current_right_ptr = self.space.len() - 1;

        while current_right_ptr > 0 {
            if let Block::Free(_) = self.space[current_right_ptr] {
                current_right_ptr -= 1;
                continue;
            }

            // find a free space position to insert
            let mut tmp_left_ptr = 0;
            // remember the free space needed
            let mut space_freed = 0;
            let mut new_space_needed = 0;
            while tmp_left_ptr < current_right_ptr {
                if let Block::Free(free_space) = self.space[tmp_left_ptr] {
                    if let Block::File(FileBlock { length, .. }) = self.space[current_right_ptr] {
                        if free_space >= length {
                            space_freed = length;
                            new_space_needed = free_space - length;
                            break;
                        }
                    }
                }
                tmp_left_ptr += 1;
            }

            // skip, if the ptr are not right placed, which happens if not enough free space is there
            if tmp_left_ptr < current_right_ptr {
                // split a free space into two chunks. One equal to searched space length and one for the rest, if any needed

                self.space.swap(tmp_left_ptr, current_right_ptr);
                self.space[current_right_ptr] = Block::Free(space_freed);
                if new_space_needed > 0 {
                    self.space
                        .insert(tmp_left_ptr + 1, Block::Free(new_space_needed));
                }
            }

            current_right_ptr -= 1;
        }
        ReducedDisk { disk: self }
    }
}

impl FromStr for Disk {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let res = s
            .char_indices()
            .map(|(i, c)| {
                let n: usize = c.to_digit(10).unwrap() as usize;
                if i % 2 == 0 {
                    Block::File(FileBlock {
                        id: i / 2,
                        length: n,
                    })
                } else {
                    Block::Free(n)
                }
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
            .flat_map(|b| match b {
                Block::Free(n) => (0..*n).map(|_| Block::Free(*n)).collect(),
                Block::File(FileBlock { id, length }) => (0..*length)
                    .map(|_| Block::File(FileBlock { id: *id, length: 1 }))
                    .collect::<Vec<Block>>(),
            })
            .enumerate()
            .map(|(i, n)| {
                if let Block::File(num) = n {
                    i * num.id
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

fn part2(input: &str) -> usize {
    Disk::from_str(input).unwrap().defragmantation().checksum()
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
        assert_eq!(part2(INPUT), 2858);
    }
}
