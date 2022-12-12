use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fmt::{Debug, Formatter};

type Height = usize;
type Index = (usize, usize);
type Point = (Index, Height);

struct HeightMap {
    heights: Vec<Vec<Height>>,
    start: Index,
    end: Index,
}

impl Debug for HeightMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for row in &self.heights {
            for height in row {
                write!(f, "{}", height)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl TryFrom<&str> for HeightMap {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut heights = Vec::new();
        let mut start = None;
        let mut end = None;

        for (i, line) in value.lines().enumerate() {
            let mut row = Vec::new();
            for (j, c) in line.chars().enumerate() {
                match c {
                    'S' => {
                        if start.is_some() {
                            return Err("Multiple start points");
                        }
                        start = Some((i, j));
                        row.push('a');
                    }
                    'E' => {
                        if end.is_some() {
                            return Err("Multiple end points");
                        }
                        end = Some((i, j));
                        row.push('z');
                    }
                    v => row.push(v),
                }
            }
            heights.push(row);
        }

        Ok(HeightMap {
            heights: heights
                .iter()
                .map(|row| row.iter().map(|&c| c as usize - 'a' as usize).collect())
                .collect(),
            start: start.ok_or("No start point")?,
            end: end.ok_or("No end point")?,
        })
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: Index,
}

impl PartialOrd<Self> for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl HeightMap {
    fn dijkstra(
        &self,
        is_end: Box<dyn Fn(&HeightMap, &Index) -> bool>,
        can_move: Box<dyn Fn(&HeightMap, &Index, &Index) -> bool>,
    ) -> (Vec<usize>, Vec<Option<Index>>, Index) {
        let row_len = self.heights.len();
        let col_len = self.heights[0].len();

        let mut dist: Vec<usize> = (0..row_len * col_len).map(|_| usize::MAX).collect();
        let mut prev: Vec<Option<Index>> = (0..row_len * col_len).map(|_| None).collect();
        let mut heap = BinaryHeap::new();

        dist[self.start.0 * col_len + self.start.1] = 0;

        heap.push(State {
            cost: 0,
            position: self.start,
        });

        while let Some(State { cost, position }) = heap.pop() {
            for neighbor in self.get_adjacent(&position) {
                let neighbor_index = neighbor.0 * col_len + neighbor.1;
                let position_height = self.heights[position.0][position.1];
                let neighbor_height = self.heights[neighbor.0][neighbor.1];

                let alt = cost
                    .checked_add(
                        self.get_distance(
                            &(position, position_height),
                            &(neighbor, neighbor_height),
                        ),
                    )
                    .unwrap_or(usize::MAX);

                if (can_move(&self, &position, &neighbor)) && alt < dist[neighbor_index] {
                    dist[neighbor_index] = alt;
                    prev[neighbor_index] = Some(position);

                    if is_end(&self, &neighbor) {
                        return (dist, prev, neighbor);
                    }

                    heap.push(State {
                        cost: alt,
                        position: neighbor,
                    });
                }
            }
        }

        (dist, prev, self.end)
    }

    fn get_adjacent(&self, index: &Index) -> Vec<Index> {
        let (i, j) = *index;
        let mut v = Vec::new();

        if i > 0 {
            v.push((i - 1, j));
        }
        if j > 0 {
            v.push((i, j - 1));
        }
        if i < self.heights.len() - 1 {
            v.push((i + 1, j));
        }
        if j < self.heights[i].len() - 1 {
            v.push((i, j + 1));
        }

        v
    }

    fn get_distance(&self, start: &Point, stop: &Point) -> usize {
        let (start_index, _start_height) = *start;
        let (stop_index, _stop_height) = *stop;

        let (start_i, start_j) = start_index;
        let (stop_i, stop_j) = stop_index;

        start_i.abs_diff(stop_i) + start_j.abs_diff(stop_j)
    }
}

fn day12_part1(text: &str) -> usize {
    let map = HeightMap::try_from(text).unwrap();

    let mut count = 0;
    let mut current = map.end.clone();
    let (_, prev, _) = map.dijkstra(
        Box::new(|map, &index| map.end == index),
        Box::new(|map, &position, &neighbor| {
            let position_height = map.heights[position.0][position.1];
            let neighbor_height = map.heights[neighbor.0][neighbor.1];

            position_height >= neighbor_height || neighbor_height - position_height <= 1
        }),
    );

    while let Some(prev) = prev[current.0 * map.heights[0].len() + current.1] {
        current = prev;
        count += 1;
    }

    count
}

fn day12_part2(text: &str) -> usize {
    let mut map = HeightMap::try_from(text).unwrap();
    map.start = map.end;

    let mut count = 0;
    let (_, prev, mut current) = map.dijkstra(
        Box::new(|map, &index| map.heights[index.0][index.1] == 0),
        Box::new(|map, &position, &neighbor| {
            let position_height = map.heights[position.0][position.1];
            let neighbor_height = map.heights[neighbor.0][neighbor.1];
            neighbor_height >= position_height - 1
        }),
    );

    while let Some(prev) = prev[current.0 * map.heights[0].len() + current.1] {
        current = prev;
        count += 1;
    }

    count
}

pub fn day12(text: &str) {
    println!("day12 part 1: {}", day12_part1(text));
    println!("day12 part 2: {}", day12_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day12_part1, day12_part2};

    const TEST_INPUT: &str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day12_part1(TEST_INPUT), 31);
    }

    #[test]
    fn fixed_tests3() {
        assert_eq!(day12_part1("SaE"), 0);
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day12_part2(TEST_INPUT), 29);
    }
}
