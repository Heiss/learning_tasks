use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::mem::swap;
use std::str::FromStr;

use itertools::Itertools;

struct DSU<T> {
    nodes: Vec<DSUNode<T>>,
    num_sets: usize,
    distances: BinaryHeap<Reverse<(usize, usize, usize)>>,
}

#[derive(Debug)]
struct DSUNode<T> {
    rank: usize,
    parent: usize,
    value: T,
    count_children: usize,
}

impl<T> DSUNode<T> {
    fn create(value: T, self_index: usize) -> DSUNode<T> {
        Self {
            rank: 0,
            parent: self_index,
            count_children: 1,
            value,
        }
    }
}

trait Distance<T> {
    fn distance(&self, o: &T) -> usize;
    fn get_x(&self) -> usize;
}

type Point3 = (usize, usize, usize);

impl Distance<Point3> for Point3 {
    fn distance(&self, o: &Point3) -> usize {
        (self.0.abs_diff(o.0)).pow(2)
            + (self.1.abs_diff(o.1)).pow(2)
            + (self.2.abs_diff(o.2)).pow(2)
    }

    fn get_x(&self) -> usize {
        self.0
    }
}

impl<T: Distance<T> + std::fmt::Debug + Copy> DSU<T> {
    fn new() -> DSU<T> {
        DSU {
            nodes: Vec::new(),
            num_sets: 0,
            distances: BinaryHeap::new(),
        }
    }

    fn insert(&mut self, node: T) {
        self.make_set(node);
    }

    fn calculate_distances(&mut self) {
        self.distances = (0..self.nodes.len())
            .combinations(2)
            .map(|pair| {
                let i = pair[0];
                let j = pair[1];
                let node_i_value = &self.nodes[i].value;
                let node_j_value = &self.nodes[j].value;
                let distance = node_i_value.distance(node_j_value);
                Reverse((distance, i, j))
            })
            .collect();
    }

    fn pop_smallest_distances(&mut self) -> Option<(usize, usize, usize)> {
        self.distances.pop().map(|r| (r.0.1, r.0.2, r.0.0))
    }

    fn make_set(&mut self, value: T) {
        let index = self.nodes.len();
        let node = DSUNode::create(value, index);
        self.nodes.push(node);
        self.num_sets += 1;
    }

    fn union(&mut self, i: usize, j: usize) {
        let mut root_i = self.find_set(i);
        let mut root_j = self.find_set(j);

        if root_i != root_j {
            if self.nodes[root_i].rank < self.nodes[root_j].rank {
                swap(&mut root_i, &mut root_j);
            }
            self.nodes[root_j].parent = root_i;
            self.nodes[root_i].count_children += self.nodes[root_j].count_children;

            if self.nodes[root_i].rank == self.nodes[root_j].rank {
                self.nodes[root_i].rank += 1;
            }
            self.num_sets -= 1;
        }
    }

    fn find_set(&mut self, i: usize) -> usize {
        if self.nodes[i].parent == i {
            return i;
        }
        self.nodes[i].parent = self.find_set(self.nodes[i].parent);
        self.nodes[i].parent
    }

    fn get_set_sizes(&mut self) -> Vec<usize> {
        (0..self.nodes.len()).for_each(|i| {
            self.find_set(i);
        });
        self.nodes
            .iter()
            .enumerate()
            .filter(|(i, node)| node.parent == *i)
            .map(|(_, node)| node.count_children)
            .collect()
    }
}

impl FromStr for DSU<Point3> {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut nodes = DSU::new();

        for line in s.lines() {
            let mut split = line.split(',');
            let value = (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            );
            nodes.insert(value);
        }
        Ok(nodes)
    }
}

fn part1(input: &str, num: usize) -> usize {
    let mut nodes = DSU::from_str(input).unwrap();
    nodes.calculate_distances();
    for _i in 0..num {
        let (i, j, _distance) = nodes.pop_smallest_distances().unwrap();
        nodes.union(i, j);
    }
    let mut set_sizes = nodes.get_set_sizes();
    set_sizes.sort_unstable_by(|a, b| b.cmp(a));
    set_sizes.iter().take(3).product()
}

fn part2(input: &str) -> usize {
    let mut nodes = DSU::from_str(input).unwrap();
    nodes.calculate_distances();
    let mut last_pop = None;
    while nodes.num_sets > 1 {
        let (i, j, distance) = nodes.pop_smallest_distances().unwrap();
        last_pop = Some((i, j, distance));
        nodes.union(i, j);
    }
    let (x_idx, y_idx, _) = last_pop.unwrap();
    let x = nodes.nodes[x_idx].value;
    let y = nodes.nodes[y_idx].value;

    x.get_x() * y.get_x()
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day8.txt");
    format!(
        "Day 8\tPart 1: {}\t Part 2: {}",
        part1(input, 1000),
        part2(input)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"#;

    #[test]
    fn day1() {
        assert_eq!(part1(INPUT, 10), 40);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 25272);
    }
}