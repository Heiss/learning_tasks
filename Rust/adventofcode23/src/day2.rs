struct Show {
    blue: usize,
    red: usize,
    green: usize,
}

impl From<&str> for Show {
    fn from(input: &str) -> Self {
        let mut blue = 0;
        let mut red = 0;
        let mut green = 0;
        let split = input.split(", ").collect::<Vec<&str>>();
        for show in split {
            let show_split = show.split(" ").collect::<Vec<&str>>();
            match show_split[1] {
                "blue" => blue = show_split[0].parse::<usize>().unwrap(),
                "red" => red = show_split[0].parse::<usize>().unwrap(),
                "green" => green = show_split[0].parse::<usize>().unwrap(),
                _ => panic!("Unknown color"),
            }
        }
        Show { blue, red, green }
    }
}

struct Game {
    id: usize,
    shows: Vec<Show>,
}

impl Game {
    fn is_valid_game(&self, show: &Show) -> bool {
        self.shows
            .iter()
            .all(|s| s.blue <= show.blue && s.red <= show.red && s.green <= show.green)
    }

    fn get_fewest_number_for_valid_game(&self) -> Show {
        let mut show = Show {
            blue: 0,
            red: 0,
            green: 0,
        };
        for s in &self.shows {
            if s.blue > show.blue {
                show.blue = s.blue;
            }
            if s.red > show.red {
                show.red = s.red;
            }
            if s.green > show.green {
                show.green = s.green;
            }
        }
        show
    }

    fn get_power(&self) -> usize {
        let show = self.get_fewest_number_for_valid_game();
        show.blue * show.red * show.green
    }
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let split = input.split(": ").collect::<Vec<&str>>();
        let id = split[0].split(" ").collect::<Vec<&str>>()[1]
            .parse::<usize>()
            .unwrap();
        let shows = split[1].split("; ").map(Show::from).collect::<Vec<Show>>();

        Game { id, shows }
    }
}

struct Games {
    games: Vec<Game>,
}

impl From<&str> for Games {
    fn from(input: &str) -> Self {
        let games = input.lines().map(Game::from).collect::<Vec<Game>>();
        Games { games }
    }
}

impl Games {
    fn calculate_id_sum(&self, shows: &Show) -> usize {
        self.games
            .iter()
            .filter(|g| g.is_valid_game(shows))
            .map(|g| g.id)
            .sum()
    }

    fn calculate_power_sum(&self) -> usize {
        self.games.iter().map(|g| g.get_power()).sum()
    }
}

fn part1(input: &str) -> usize {
    let games = Games::from(input);
    let show = Show::from("12 red, 13 green, 14 blue");
    games.calculate_id_sum(&show)
}

fn part2(input: &str) -> usize {
    let games = Games::from(input);
    games.calculate_power_sum()
}

pub fn day() {
    let input = include_str!("../input/day2.txt");
    println!(
        "Day 2\t Part 1: {}\t Part 2: {}",
        part1(input),
        part2(input)
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"#;
    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 8);
    }

    #[test]
    fn test_test2() {
        assert_eq!(part2(INPUT), 2286);
    }
}
