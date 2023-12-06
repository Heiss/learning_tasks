use std::str::FromStr;

#[derive(Debug)]
struct Race {
    distance: usize,
    time: usize,
}

impl Race {
    fn from_str(time: &str, distance: &str) -> Self {
        Self {
            distance: distance.parse().unwrap(),
            time: time.parse().unwrap(),
        }
    }

    fn get_solution_range(&self) -> (usize, usize) {
        let distance = self.distance as f64;
        let time = self.time as f64;

        let right = (time.powi(2) - 4.0 * distance).sqrt();
        let down_border = (time - right) / 2.0;
        let upper_border = (time + right) / 2.0;

        ((down_border + 1.0).floor() as usize, (upper_border - 1.0).ceil() as usize)
    }

    fn get_count_solutions(&self) -> usize {
        let (down_border, upper_border) = self.get_solution_range();
        upper_border - down_border + 1
    }
}

#[derive(Debug)]
struct Races(Vec<Race>);

impl Races {
    fn get_number_of_solutions(&self) -> usize {
        self.0.iter().map(|r| r.get_count_solutions()).fold(1, |acc, x| acc * x)
    }
}

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines = s.lines().collect::<Vec<&str>>();
        let times = lines[0].split_whitespace().skip(1);
        let distances = lines[1].split_whitespace().skip(1);

        Ok(Self(times.zip(distances).map(|(t, d)| Race::from_str(t, d)).collect()))
    }
}

fn part1(input: &str) -> usize {
    let races = Races::from_str(input).unwrap();
    races.get_number_of_solutions()
}

fn part2(input: &str) -> usize {
    let input = input.replace(" ", "").replace(":", ": ");

    let races = Races::from_str(&input).unwrap();
    races.get_number_of_solutions()
}

pub fn day() {
    let input = include_str!("../input/day6.txt");
    print!("Day 6\t");
    print!("Part 1: {}\t", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"Time: 7  15   30
Distance: 9  40  200"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 288);
    }

    #[test]
    fn test_ranges() {
        let races = Races::from_str("Time: 7\nDistance: 9").unwrap();
        assert_eq!(races.0.get(0).unwrap().get_solution_range(), (2, 5));
        assert_eq!(races.get_number_of_solutions(), 4);
    }

    #[test]
    fn test_ranges2() {
        let races = Races::from_str("Time: 30\nDistance: 200").unwrap();
        assert_eq!(races.0.get(0).unwrap().get_solution_range(), (11, 19));
        assert_eq!(races.get_number_of_solutions(), 9);
    }

    #[test]
    fn test_ranges3() {
        let races = Races::from_str("Time: 15\nDistance: 40").unwrap();
        assert_eq!(races.get_number_of_solutions(), 8);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 71503);
    }
}
