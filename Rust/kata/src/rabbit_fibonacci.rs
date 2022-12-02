use num::{bigint::BigUint, One, Zero};

struct Rabbit {
    mature: BigUint,
    immature: BigUint,
    rate: BigUint,
}

impl Rabbit {
    fn new(rate: u8) -> Self {
        Self {
            mature: One::one(),
            immature: Zero::zero(),
            rate: BigUint::from(rate),
        }
    }

    fn tick(&mut self) {
        let next_birth = &self.rate * &self.immature;
        self.immature += &self.mature;
        self.mature = next_birth;
        println!("count: {:?}", self.immature);
    }
}

pub fn fib_rabbits(n: u8, b: u8) -> BigUint {
    println!("rate: {}, months: {}", n, b);
    let mut rabbits = Rabbit::new(b);
    (0..n).for_each(|i| {
        print!("tick: {} ", i);
        rabbits.tick()
    });

    rabbits.immature
}

// Add your tests here.
// See https://doc.rust-lang.org/stable/rust-by-example/testing/unit_testing.html

#[cfg(test)]
mod tests {
    use super::fib_rabbits;
    use num::bigint::BigUint;
    use num::bigint::ToBigUint;
    use num::traits::One;
    use num::traits::Zero;

    #[test]
    fn sample_tests() {
        assert_eq!(fib_rabbits(0, 4), BigUint::zero());
        assert_eq!(fib_rabbits(1, 4), BigUint::one());
        assert_eq!(fib_rabbits(4, 0), BigUint::one());
        assert_eq!(fib_rabbits(6, 3), 40.to_biguint().unwrap());
        assert_eq!(fib_rabbits(8, 12), 8425.to_biguint().unwrap());
        assert_eq!(fib_rabbits(7, 4), 181.to_biguint().unwrap());
    }
}
