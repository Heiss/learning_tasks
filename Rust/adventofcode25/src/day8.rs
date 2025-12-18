use std::fmt::Display;
use std::mem::swap;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use itertools::Itertools;

struct DSU<T> {
    nodes: Vec<Arc<Mutex<DSUNode<T>>>>,
    sets: Vec<Arc<Mutex<DSUNode<T>>>>,
    distances: Vec<(usize, usize, usize)>,
}

#[derive(Debug)]
struct DSUNode<T> {
    rank: usize,
    parent: Option<Arc<Mutex<DSUNode<T>>>>,
    value: T,
    count_children: usize,
}

impl<T> DSUNode<T> {
    fn create(value: T) -> DSUNode<T> {
        Self {
            rank: 0,
            parent: None,
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
        ((self.0.abs_diff(o.0)).pow(2)
            + (self.1.abs_diff(o.1)).pow(2)
            + (self.2.abs_diff(o.2)).pow(2))
        .isqrt()
    }

    fn get_x(&self) -> usize {
        self.0
    }
}

impl<T: Distance<T> + std::fmt::Debug> DSU<T> {
    fn new() -> DSU<T> {
        DSU {
            nodes: Vec::new(),
            sets: Vec::new(),
            distances: Vec::new(),
        }
    }

    fn insert(&mut self, node: T) {
        self.make_set(node);
    }

    fn calculate_distances(&mut self) {
        let mut distances: Vec<(usize, usize, usize)> = (0..self.nodes.len())
            .combinations(2)
            .map(|pair| {
                let i = pair[0];
                let j = pair[1];
                let node_i_value = &self.nodes[i].lock().unwrap().value;
                let node_j_value = &self.nodes[j].lock().unwrap().value;
                let distance = node_i_value.distance(node_j_value);
                (i, j, distance)
            })
            .collect();

        // Sort by distance in descending order.
        distances.sort_by(|a, b| b.2.cmp(&a.2));

        self.distances = distances;
    }

    fn pop_smallest_distances(&mut self) -> Option<(usize, usize, usize)> {
        self.distances.pop()
    }

    fn make_set(&mut self, value: T) -> Arc<Mutex<DSUNode<T>>> {
        let node = Arc::new(Mutex::new(DSUNode::create(value)));
        node.lock().unwrap().parent = Some(node.clone());
        self.nodes.push(node.clone());
        self.sets.push(node.clone());
        node
    }

    fn union(&mut self, a: Arc<Mutex<DSUNode<T>>>, b: Arc<Mutex<DSUNode<T>>>) {
        self.link(self.find_set(a), self.find_set(b));
    }

    fn remove(&mut self, node: Arc<Mutex<DSUNode<T>>>) {
        for i in 0..self.sets.len() {
            if Arc::ptr_eq(&node, &self.sets[i]) {
                self.sets.remove(i);
                break;
            }
        }
    }

    fn link(&mut self, a: Arc<Mutex<DSUNode<T>>>, b: Arc<Mutex<DSUNode<T>>>) {
        let mut a = self.find_set(a);
        let mut b = self.find_set(b);

        if !Arc::ptr_eq(&a, &b) {
            let (a_rank, b_rank) = (a.lock().unwrap().rank, b.lock().unwrap().rank);
            if a_rank < b_rank {
                swap(&mut a, &mut b);
            }
            b.lock().unwrap().parent = Some(a.clone());
            a.lock().unwrap().count_children += b.lock().unwrap().count_children;
            self.remove(b.clone());

            if a_rank == b_rank {
                a.lock().unwrap().rank += 1;
            }
        }
    }

    fn find_set(&self, p0: Arc<Mutex<DSUNode<T>>>) -> Arc<Mutex<DSUNode<T>>> {
        let mut n = p0.lock().unwrap();
        let parent = n.parent.as_ref().unwrap().clone();
        if Arc::ptr_eq(&parent, &p0) {
            return p0.clone();
        }
        n.parent = Some(self.find_set(parent));
        n.parent.as_ref().unwrap().clone()
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
        let (i, j, _distance) = &nodes.pop_smallest_distances().unwrap();
        let nearest = nodes.nodes[*i].clone();
        let current = nodes.nodes[*j].clone();
        nodes.union(nearest, current);
    }
    nodes.sets.sort_by(|a, b| {
        a.lock()
            .unwrap()
            .count_children
            .cmp(&b.lock().unwrap().count_children)
    });
    nodes
        .sets
        .iter()
        .rev()
        .take(3)
        .fold(1, |a, b| a * b.lock().unwrap().count_children)
}

fn part2(input: &str) -> usize {
    let mut nodes = DSU::from_str(input).unwrap();
    nodes.calculate_distances();
    let mut last_pop = None;
    while nodes.sets.len() > 1 {
        last_pop = nodes.pop_smallest_distances();
        let (i, j, _distance) = last_pop.unwrap();
        let nearest = nodes.nodes[i].clone();
        let current = nodes.nodes[j].clone();
        nodes.union(nearest, current);
    }
    let x = last_pop.unwrap().0;
    let y = last_pop.unwrap().1;
    let x = nodes.nodes[x].clone().lock().unwrap().value;
    let y = nodes.nodes[y].clone().lock().unwrap().value;

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
