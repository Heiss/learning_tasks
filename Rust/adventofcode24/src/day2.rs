use std::str::FromStr;

type Level = usize;
struct Report {
    levels: Vec<Level>,
}
struct Reports(Vec<Report>);

impl FromStr for Report {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            levels: s
                .split_whitespace()
                .map(|v| v.parse::<usize>())
                .map(Result::unwrap)
                .collect(),
        })
    }
}

impl FromStr for Reports {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Reports(
            s.lines()
                .map(Report::from_str)
                .map(Result::unwrap)
                .collect(),
        ))
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        Self::calculate_safeness(&self.levels)
    }

    fn calculate_safeness(levels: &Vec<Level>) -> bool {
        let list = levels.windows(2).map(|w| match w {
            [x, y] => *x as isize - *y as isize,
            _ => panic!("Not possible"),
        });

        let decreasing_list = list.clone().filter(|x| *x > 0 && *x < 4).count();
        let increasing_list = list.clone().filter(|x| *x < 0 && *x > (-4)).count();
        let size = list.count();

        increasing_list == size && decreasing_list == 0
            || decreasing_list == size && increasing_list == 0
    }

    fn is_dampener_safe(&self) -> bool {
        let mut dampened_levels = Vec::new();
        dampened_levels.push(self.levels.clone());

        for i in 0..self.levels.iter().count() {
            let mut new_list = self.levels.clone();
            new_list.remove(i);
            dampened_levels.push(new_list);
        }

        dampened_levels.iter().map(|x| Self::calculate_safeness(x)).filter(|x| *x).count() > 0
    }
}

impl Reports {
    fn get_count_of_safe_reports(&self) -> usize {
        self.0.iter().filter(|x| x.is_safe()).count()
    }

    fn get_count_of_reports_dampener(&self) -> usize {
        self.0.iter().filter(|x| x.is_dampener_safe()).count()
    }
}

fn part1(input: &str) -> usize {
    Reports::from_str(input)
        .unwrap()
        .get_count_of_safe_reports()
}

fn part2(input: &str) -> usize {
    let rep = Reports::from_str(input).unwrap();
    rep.get_count_of_reports_dampener()
}

pub fn day() -> String {
    let input = include_str!("../input/day2.txt");
    format!("Day 2\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day1() {
        const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part1(INPUT), 2);
    }
    #[test]
    fn day2() {
        const INPUT: &str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

        assert_eq!(part2(INPUT), 4);
    }
}
