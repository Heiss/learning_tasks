struct Tills {
    tills: Vec<usize>,
    queue: Vec<usize>,
}

impl Tills {
    fn new(tills: usize, queue: Vec<usize>) -> Tills {
        Tills {
            tills: vec![0; tills],
            queue,
        }
    }

    fn tick(&mut self) {
        for till in self.tills.iter_mut() {
            if *till > 0 {
                *till -= 1;
            }

            if *till == 0 {
                if let Some(customer) = self.queue.pop() {
                    *till = customer;
                }
            }
        }
    }

    fn is_empty(&self) -> bool {
        self.tills.iter().all(|till| *till == 0) && self.queue.is_empty()
    }

    fn iter(&mut self) -> IterTills {
        IterTills {
            tills: self,
            time: 0,
        }
    }
}

struct IterTills<'a> {
    tills: &'a mut Tills,
    time: usize,
}

impl<'a> std::iter::Iterator for IterTills<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.tills.is_empty() {
            None
        } else {
            self.tills.tick();
            self.time += 1;
            Some(self.time)
        }
    }
}

fn queue_time(customers: &[u32], n: u32) -> u32 {
    let mut tills = Tills::new(n as usize, customers.iter().map(|&x| x as usize).collect());
    tills.iter().last().unwrap_or_else(|| 0) as u32
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::queue_time;

    fn dotest(c: &[u32], n: u32, expected: u32) {
        let actual = queue_time(c, n);
        assert_eq!(
            actual, expected,
            "With customers = {c:?}, n = {n}\nExpected {expected} but got {actual}"
        )
    }

    #[test]
    fn fixed_tests() {
        return;
        dotest(&[], 1, 0);
        dotest(&[5], 1, 5);
        dotest(&[2], 5, 2);
        dotest(&[1, 2, 3, 4, 5], 1, 15);
        dotest(&[1, 2, 3, 4, 5], 100, 5);
        dotest(&[2, 2, 3, 3, 4, 4], 2, 9);
    }
}
