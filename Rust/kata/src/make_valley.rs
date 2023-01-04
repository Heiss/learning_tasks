struct Valley {
    left: Vec<i32>,
    right: Vec<i32>,
}

impl From<Vec<i32>> for Valley{
    fn from(mut vec: Vec<i32>) -> Self {
        vec.sort();
        let mut left = Vec::new();
        let mut right = Vec::new();

        vec.chunks(2).for_each(|chunk| {
            match chunk {
                [a] => left.push(*a),
                [a, b] => {
                    left.push(*a);
                    right.push(*b);
                }
                _ => unreachable!(),
            }
        });

        Self { left, right }
    }
}

impl Valley{
    fn consume(mut self) -> Vec<i32> {
        let mut result = Vec::new();
        self.left.reverse();
        result.append(&mut self.left);
        result.append(&mut self.right);

        if result.len() % 2 == 0 {
            result.reverse();
        }

        result
    }
}

// Task https://www.codewars.com/kata/56e3cd1d93c3d940e50006a4
fn make_valley(arr: Vec<i32>) -> Vec<i32> {
    Valley::from(arr).consume()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(arr: Vec<i32>, exp: Vec<i32>) -> () {
        println!("arr: {:?}", arr);
        let ans = make_valley(arr);
        println!("actual:\n{:?}", ans);
        println!("expect:\n{:?}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(vec![17, 17, 15, 14, 8, 7, 7, 5, 4, 4, 1], vec![17, 15, 8, 7, 4, 1, 4, 5, 7, 14, 17]);
        dotest(vec![20, 7, 6, 2], vec![20, 6, 2, 7]);
    }
}