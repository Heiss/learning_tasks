// https://www.codewars.com/kata/52c31f8e6605bcc646000082/train/rust

use std::collections::HashMap;
use thiserror::Error;

type Solution = (PositionInGivenArray, PositionInGivenArray);
type Result<T> = std::result::Result<T, FinderError>;
type FindNumber = i32;
type GivenNumber = i32;
type PositionInGivenArray = usize;

#[derive(Debug, Error)]
enum FinderError {
    #[error("No solution found")]
    NotFound,
}

struct FinderBuilder<'a> {
    numbers: &'a [GivenNumber],
    target: &'a FindNumber,
}

impl<'a> FinderBuilder<'a> {
    fn new(numbers: &'a [GivenNumber], target: &'a FindNumber) -> Self {
        Self { numbers, target }
    }

    /// Create a finder
    /// Make some heavy stuff.
    fn create_finder(&self) -> Finder {
        let mut hashMap = HashMap::new();

        for (index, number) in self.numbers.iter().enumerate() {
            let entry = hashMap.entry(number).or_insert_with(Vec::new);
            entry.push(index);
        }

        Finder {
            hashMap,
            target: self.target,
        }
    }
}

struct Finder<'a> {
    hashMap: HashMap<&'a GivenNumber, Vec<PositionInGivenArray>>,
    target: &'a FindNumber,
}

impl<'a> Finder<'a> {
    fn find(&self) -> Result<Solution> {
        let mut solution = None;

        for (&current_number, position_of_current_number) in self.hashMap.iter() {
            let difference: FindNumber = self.target - current_number;

            // falls difference diesselbe Zahl ergibt, dann muss die Zahl mindestens 2 mal vorkommen,
            // sonst gibts kein Ergebnis
            if *current_number == difference {
                if position_of_current_number.len() > 1 {
                    solution = Some((position_of_current_number[0], position_of_current_number[1]));
                    break;
                }

            // falls difference eine andere Zahl ergibt, dann muss die Zahl mindestens 1 mal vorkommen,
            // wenn get_index ein Ergebnis zurückgibt. Uns ist dann die Position auch egal.
            } else if let Some(j) = self.get_index(&difference) {
                solution = Some((position_of_current_number[0], j[0]));
                break;
            }
        }

        solution.ok_or(FinderError::NotFound)
    }

    fn get_index(&self, number: &FindNumber) -> Option<Vec<PositionInGivenArray>> {
        self.hashMap.get(&number).map(|v| v.clone())
    }
}

fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    FinderBuilder::new(numbers, &target)
        .create_finder()
        .find()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::two_sum;

    #[test]
    fn sample() {
        do_test(&[1, 2, 3], 4);
        do_test(&[1235, 5678, 9012], 14690);
        do_test(&[2, 2, 3], 4);
    }

    fn do_test(nums: &[i32], sum: i32) {
        let len = nums.len();
        let user_tuple = two_sum(nums, sum);
        assert!(
            user_tuple.0 < len && user_tuple.1 < len,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple has an index out of bounds",
            nums,
            sum,
            user_tuple
        );
        assert!(
            user_tuple.0 != user_tuple.1,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple must have two different indices",
            nums, sum, user_tuple
        );
        let num1 = nums[user_tuple.0];
        let num2 = nums[user_tuple.1];
        let user_sum = num1 + num2;
        assert!(
            user_sum == sum,
            "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nnumber as index {}: {}\nnumber as index {}: {}\nsum of the two numbers: {}\nsum of the two numbers did not equal target",
            nums, sum, user_tuple, user_tuple.0, num1, user_tuple.1, num2, user_sum
        )
    }
}
