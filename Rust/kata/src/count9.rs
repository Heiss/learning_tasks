fn count_nines(n: u64) -> u64 {
    counter::count(n, 9)
}

mod counter {
    use std::cmp::Ordering;

    pub fn count(n: u64, searched_number: u64) -> u64 {
        if n < searched_number {
            return 0;
        }

        let d = (n as f64).log(10.0) as usize;
        let mut arr = vec![1; d + 1];

        arr[0] = 0;
        (2..d + 1).for_each(|i| {
            arr[i] = arr[i - 1] * 9 + 10_u64.pow(u32::try_from(i).unwrap() - 1) + arr[i - 1]
        });

        let p = 10_u64.pow(u32::try_from(d).unwrap());
        let msd = n / p;

        println!("d={d}, n={n}, arr[d]={}, p={p}, msd={msd}", arr[d]);

        match msd.cmp(&searched_number) {
            Ordering::Equal => msd * arr[d] + (n % p) + 1,
            Ordering::Greater => (msd - 1) * arr[d] + p + count(n % p, searched_number),
            Ordering::Less => msd * arr[d] + count(n % p, searched_number),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::count_nines;

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    #[test]
    fn basic() {
        assert_eq!(count_nines(1), 0, "Nein!{ERR_MSG} with n = 1");
        assert_eq!(count_nines(9), 1, "Nein!{ERR_MSG} with n = 9");
        assert_eq!(count_nines(10), 1, "Nein!{ERR_MSG} with n = 10");
        assert_eq!(count_nines(98), 18, "Nein!{ERR_MSG} with n = 98");
        assert_eq!(count_nines(100), 20, "Nein!{ERR_MSG} with n = 100");
        assert_eq!(
            count_nines(565_754),
            275_645,
            "Nein!{ERR_MSG} with n = 565,754"
        );
        assert_eq!(
            count_nines(10_000_000_000),
            10_000_000_000,
            "Nein!{ERR_MSG} with n = 10,000,000,000"
        );
    }
}
