use std::cmp::Ordering;

#[derive(Debug)]
enum PointsError {
    InvalidFormat,
    InvalidNumber,
}

#[derive(Debug)]
struct Points(u8, u8);

impl Points {
    pub fn new(own_score: u8, enemy_score: u8) -> Points {
        Self(own_score, enemy_score)
    }

    fn points(&self) -> u32 {
        match self.0.cmp(&self.1) {
            Ordering::Less => 0,
            Ordering::Equal => 1,
            Ordering::Greater => 3,
        }
    }
}
impl TryFrom<&str> for Points {
    type Error = PointsError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let position = match value.find(':') {
            None => return Err(PointsError::InvalidFormat),
            Some(v) => v,
        };

        let own_score = value[0..position]
            .parse()
            .map_err(|_| PointsError::InvalidNumber)?;
        let enemy_score = value[position + 1..]
            .parse()
            .map_err(|_| PointsError::InvalidNumber)?;

        Ok(Points::new(own_score, enemy_score))
    }
}

pub fn points(games: &[String]) -> u32 {
    games
        .iter()
        .map(|v| Points::try_from(v.as_str()).unwrap())
        .map(|p| p.points())
        .sum()
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::points;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn do_fixed_test(e: &[&str], expected: u32) {
        let a = &e.iter().map(|x| x.to_string()).collect::<Vec<_>>();
        assert_eq!(points(a), expected, "{ERR_MSG} with games = {a:?}")
    }

    #[test]
    fn fixed_tests() {
        do_fixed_test(
            &[
                "1:0", "2:0", "3:0", "4:0", "2:1", "3:1", "4:1", "3:2", "4:2", "4:3",
            ],
            30,
        );
        do_fixed_test(
            &[
                "1:1", "2:2", "3:3", "4:4", "2:2", "3:3", "4:4", "3:3", "4:4", "4:4",
            ],
            10,
        );
        do_fixed_test(
            &[
                "0:1", "0:2", "0:3", "0:4", "1:2", "1:3", "1:4", "2:3", "2:4", "3:4",
            ],
            0,
        );
        do_fixed_test(
            &[
                "1:0", "2:0", "3:0", "4:0", "2:1", "1:3", "1:4", "2:3", "2:4", "3:4",
            ],
            15,
        );
        do_fixed_test(
            &[
                "1:0", "2:0", "3:0", "4:4", "2:2", "3:3", "1:4", "2:3", "2:4", "3:4",
            ],
            12,
        );
    }
}
