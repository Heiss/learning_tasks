use std::cell::Cell;
use std::rc::Rc;
use std::sync::Mutex;

struct DSU<T> {
    nodes: Vec<Rc<Mutex<DSUNode<T>>>>,
}

struct DSUNode<T> {
    rank: usize,
    parent: Option<Rc<Mutex<DSUNode<T>>>>,
    value: T,
}

impl<T> DSU<T> {
    fn make_set(&mut self, value: T) -> Rc<Mutex<DSUNode<T>>> {
        let node = Rc::new(Mutex::new(DSUNode {
            rank: 0,
            parent: None,
            value,
        }));
        let result = node.clone();
        self.nodes.push(node);
        result
    }

    fn union(&mut self, a: Rc<Mutex<DSUNode<T>>>, b: Rc<Mutex<DSUNode<T>>>) {
        self.link(self.find_set(a), self.find_set(b))
    }

    fn link(&self, in_x: Rc<Mutex<DSUNode<T>>>, in_y: Rc<Mutex<DSUNode<T>>>) {
        let x = in_x.lock().unwrap();
        let mut y = in_y.lock().unwrap();

        if x.rank > y.rank {
            y.parent.replace(in_x.clone());
        } else if x.parent.is_some()
            && Rc::ptr_eq(x.parent.as_ref().unwrap(), &in_y)
            && x.rank == y.rank
        {
            y.rank += 1;
        }
    }

    fn find_set(&self, p0: Rc<Mutex<DSUNode<T>>>) -> Rc<Mutex<DSUNode<T>>> {
        let mut p0 = p0.lock().unwrap();
        if let Some(p) = &p0.parent {
            p0.parent = Some(self.find_set(p.clone()));
        }
        p0.parent.clone().unwrap()
    }
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

pub fn day() -> String {
    let input = include_str!("../../../input/Rust/adventofcode25/day8.txt");
    format!("Day 8\tPart 1: {}\t Part 2: {}", part1(input), part2(input))
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
        assert_eq!(part1(INPUT), 40);
    }
    #[test]
    fn day2() {
        assert_eq!(part2(INPUT), 40);
    }
}
