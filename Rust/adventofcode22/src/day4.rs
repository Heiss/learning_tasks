#[derive(Debug)]
struct WorkingArea {
    fields: Vec<u32>,
}

#[derive(Debug)]
enum WorkingAreaError {
    InvalidInput,
}

impl TryFrom<&str> for WorkingArea {
    type Error = WorkingAreaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let split: Vec<u32> = value
            .split("-")
            .map(|v| v.parse::<u32>().expect("Invalid input"))
            .collect();

        if split.len() != 2 {
            return Err(WorkingAreaError::InvalidInput);
        }

        let fields: Vec<u32> = (split[0]..=split[1]).collect();

        Ok(WorkingArea { fields })
    }
}

impl WorkingArea {
    fn contains(&self, other: &WorkingArea) -> bool {
        self.fields
            .iter()
            .map(|v| other.fields.contains(v))
            .all(|v| v)
    }

    fn overlaps(&self, other: &WorkingArea) -> bool {
        self.fields
            .iter()
            .map(|v| other.fields.contains(v))
            .any(|v| v)
    }
}

#[derive(Debug)]
struct Shipment {
    working_areas: Vec<(WorkingArea, WorkingArea)>,
}

impl Shipment {
    fn count_contains(&self) -> u64 {
        self.working_areas
            .iter()
            .map(|(a, b)| (a.contains(b) || b.contains(a)).then(|| 1).unwrap_or(0))
            .sum()
    }

    fn count_overlaps(&self) -> u64 {
        self.working_areas
            .iter()
            .map(|(a, b)| a.overlaps(b).then(|| 1).unwrap_or(0))
            .sum()
    }
}

impl TryFrom<&str> for Shipment {
    type Error = WorkingAreaError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut split: Vec<WorkingArea> = value
            .split(",")
            .map(|v| WorkingArea::try_from(v).expect("Invalid input"))
            .collect();

        if split.len() != 2 {
            return Err(WorkingAreaError::InvalidInput);
        }

        Ok(Shipment {
            working_areas: vec![(split.swap_remove(0), split.swap_remove(0))],
        })
    }
}

fn day4_part1(text: &str) -> u64 {
    let shipments: Vec<Shipment> = text
        .lines()
        .map(|v| Shipment::try_from(v).expect("Invalid input"))
        .collect();

    shipments.iter().map(|v| v.count_contains()).sum()
}

fn day4_part2(text: &str) -> u64 {
    let shipments: Vec<Shipment> = text
        .lines()
        .map(|v| Shipment::try_from(v).expect("Invalid input"))
        .collect();

    shipments.iter().map(|v| v.count_overlaps()).sum()
}

pub fn day4(text: &str) {
    println!("Day4 part 1: {}", day4_part1(text));
    println!("Day4 part 2: {}", day4_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day4_part1, day4_part2};

    const TEST_INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day4_part1(TEST_INPUT), 2);
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day4_part2(TEST_INPUT), 4);
    }
}
