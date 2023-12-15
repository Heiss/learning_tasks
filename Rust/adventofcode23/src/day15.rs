struct HashIter<'a> {
    remainder: &'a str,
    delimiter: &'a str,
}

impl<'a> HashIter<'a> {
    fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        if delimiter.is_empty() {
            panic!("Invalid delimiter {delimiter:?}");
        }

        Self {
            remainder: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for HashIter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(next_delim) = self.remainder.find(self.delimiter) {
            let until_delimiter = &self.remainder[..next_delim];
            self.remainder = &self.remainder[next_delim + self.delimiter.len()..];

            Some(until_delimiter)
        } else if !self.remainder.is_empty() {
            let until_delimiter = self.remainder;
            self.remainder = "";

            Some(until_delimiter)
        } else {
            None
        }
    }
}

fn calc_hash(input: &str) -> u8 {
    let mut hash = 0;
    for c in input.chars() {
        hash = (hash + c as usize) * 17 % 256;
    }
    hash as u8
}

type BoxPosition = u8;
type FocalLength = u8;

#[derive(Hash, Copy, Clone, Debug)]
struct Lens<'a> {
    label: &'a str,
    focal_length: FocalLength,
}

impl PartialEq for Lens<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.label == other.label
    }
}

impl PartialEq<&str> for Lens<'_> {
    fn eq(&self, other: &&str) -> bool {
        self.label == *other
    }
}

type Label<'a> = &'a str;

enum LensOperation<'a> {
    Insert(BoxPosition, Lens<'a>),
    Remove(BoxPosition, Label<'a>),
}

impl<'a> LensOperation<'a> {
    fn from_str(input: &'a str) -> Self {
        if input.contains(',') {
            panic!("Invalid input {input:?}");
        }

        if let Some((label, focal)) = input.split_once('=') {
            let focal = focal.parse().unwrap();
            let position = calc_hash(label) as BoxPosition;

            return Self::Insert(position, Lens { label, focal_length: focal });
        } else if let Some((label, _)) = input.split_once('-') {
            let position = calc_hash(label) as BoxPosition;

            return Self::Remove(position, label);
        } else {
            panic!("Invalid input {input:?}");
        }
    }
}


fn part1(input: &str) -> usize {
    let iter = HashIter::new(input, ",");
    iter.into_iter().map(|v| calc_hash(v) as usize).sum()
}

fn part2(input: &str) -> usize {
    let iter = HashIter::new(input, ",");
    let mut boxes: [Vec<Lens>; 256] = vec![Vec::new(); 256].try_into().unwrap();

    for op in iter.into_iter().map(LensOperation::from_str) {
        match op {
            LensOperation::Insert(position, lens) => {
                let position = position as usize;
                if let Some(l) = boxes[position].iter_mut().find(|l| l == &&lens.label) {
                    l.focal_length = lens.focal_length;
                } else {
                    boxes[position].push(lens);
                };
            }
            LensOperation::Remove(position, label) => {
                let position = position as usize;
                boxes[position].retain(|lens| *lens != label);
            }
        }
    }

    boxes.iter().enumerate().flat_map(|(box_pos, bx)| bx.iter().enumerate().map(move |(lens_pos, lens)| {
        vec![box_pos + 1, lens_pos + 1, lens.focal_length as usize].iter().product::<usize>()
    })).sum()
}

pub fn day() {
    let input = include_str!("../input/day15.txt");
    print!("day 15\t");
    print!("part 1: {}\t", part1(input));
    println!("part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 1320);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 145);
    }
}
