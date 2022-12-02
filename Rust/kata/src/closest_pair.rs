fn distance_points(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

struct FoundMinimum {
    distance: f64,
    pub p1: (f64, f64),
    pub p2: (f64, f64),
}

pub fn closest_pair(points: &[(f64, f64)]) -> ((f64, f64), (f64, f64)) {
    let mut minimal_distance = FoundMinimum {
        distance: std::f64::MAX,
        p1: points[0],
        p2: points[1],
    };

    let mut seen = vec![];

    points.iter().for_each(|p1| {
        seen.iter().for_each(|p2| {
            let distance = distance_points(*p1, *p2);
            if distance < minimal_distance.distance {
                minimal_distance = FoundMinimum {
                    distance,
                    p1: *p1,
                    p2: *p2,
                };
            }
        });
        seen.push(*p1);
    });

    (minimal_distance.p1, minimal_distance.p2)
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::closest_pair;

    type Points = ((f64, f64), (f64, f64));

    fn verify(actual: Points, expected: Points) {
        if actual == expected || (actual.0 == expected.1 && actual.1 == expected.0) {
            assert!(true)
        } else {
            assert_eq!(actual, expected)
        }
    }

    #[test]
    fn sample_tests() {
        verify(
            closest_pair(&[(2.0, 2.0), (6.0, 3.0)]),
            ((2.0, 2.0), (6.0, 3.0)),
        );
        verify(
            closest_pair(&[
                (2.0, 2.0),
                (2.0, 8.0),
                (5.0, 5.0),
                (6.0, 3.0),
                (6.0, 7.0),
                (7.0, 4.0),
                (7.0, 9.0),
            ]),
            ((6.0, 3.0), (7.0, 4.0)),
        );
        verify(
            closest_pair(&[
                (2.0, 2.0),
                (2.0, 8.0),
                (5.0, 5.0),
                (5.0, 5.0),
                (6.0, 3.0),
                (6.0, 7.0),
                (7.0, 4.0),
                (7.0, 9.0),
            ]),
            ((5.0, 5.0), (5.0, 5.0)),
        );
    }
}
