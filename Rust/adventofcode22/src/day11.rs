use itertools::Itertools;

/// Target symbolize the monkey number to threw the item, whether the test returns (True, False)
type Target = (usize, usize);
#[derive(Debug, Clone)]
struct Item(usize);

#[derive(Debug, Clone)]
enum OperationTerm {
    Old,
    Constant(usize),
}

impl OperationTerm {
    fn execute(&self, old: usize) -> usize {
        match self {
            OperationTerm::Old => old,
            OperationTerm::Constant(c) => *c,
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(OperationTerm, OperationTerm),
    Mul(OperationTerm, OperationTerm),
}

impl Operation {
    fn execute(&self, old: usize) -> usize {
        match self {
            Operation::Add(a, b) => a.execute(old) + b.execute(old),
            Operation::Mul(a, b) => a.execute(old) * b.execute(old),
        }
    }
}

struct Monkey {
    items: Vec<Item>,
    target: Target,
    op: Operation,
    divisible: usize,
    counts_inspections: usize,
}

type MoneyTarget = usize;
type MonkeyThrow = (MoneyTarget, Item);
impl Monkey {
    fn inspect_and_throw(&mut self, calm_factor: &CalmFactor) -> Vec<MonkeyThrow> {
        let mut vec = vec![];

        while self.items.len() > 0 {
            let mut item = self.items.remove(0);
            self.counts_inspections += 1;

            item = match calm_factor {
                CalmFactor::Part1(v) => Item(self.op.execute(item.0) / v),
                CalmFactor::Part2(base) => Item(self.op.execute(item.0) % base),
            };

            if item.0 % self.divisible == 0 {
                vec.push((self.target.0, item));
            } else {
                vec.push((self.target.1, item));
            }
        }

        vec
    }

    fn catch(&mut self, item: &Item) {
        self.items.push(item.clone());
    }
}

impl TryFrom<&str> for Monkey {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut items = vec![];
        let mut lines = value.lines();
        let mut current = lines.next();
        let mut op = None;
        let mut divisible = None;
        let mut target: Target = (0, 0);

        while current != None {
            let line: Vec<&str> = current.unwrap().split(": ").collect();

            match line[0].trim() {
                "Starting items" => {
                    for item in line[1].split(", ") {
                        items.push(Item(
                            item.parse::<usize>()
                                .map_err(|_| "Invalid number in starting items")?,
                        ));
                    }
                }
                "Operation" => {
                    let calculation: Vec<&str> = line[1].split(" = ").collect();
                    let operation: Vec<&str> = calculation[1].split(" ").collect();
                    let left = match operation[0].parse::<usize>() {
                        Ok(value) => OperationTerm::Constant(value),
                        Err(_) => OperationTerm::Old,
                    };
                    let right = match operation[2].parse::<usize>() {
                        Ok(value) => OperationTerm::Constant(value),
                        Err(_) => OperationTerm::Old,
                    };

                    op = Some(match operation[1] {
                        "+" => Operation::Add(left, right),
                        "*" => Operation::Mul(left, right),
                        v => {
                            println!("Invalid operation: {}", v);
                            return Err("Invalid operation");
                        }
                    });
                }
                "Test" => {
                    let split: Vec<&str> = line[1].split(" by ").collect();
                    divisible = Some(split[1].parse::<usize>().map_err(|_| "Invalid number")?);
                    let line = lines.next().expect("Missing line");
                    let on_true = line
                        .split(" monkey ")
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .map_err(|_| "Invalid monkey number")?;
                    let line = lines.next().expect("Missing line");
                    let on_false = line
                        .split(" monkey ")
                        .skip(1)
                        .next()
                        .unwrap()
                        .parse::<usize>()
                        .map_err(|_| "Invalid monkey number")?;
                    target = (on_true, on_false);
                }
                v => {
                    if v.starts_with("Monkey") {
                        current = lines.next();
                        continue;
                    }
                    println!("Unknown line: {}", v);
                    return Err("Invalid line");
                }
            }
            current = lines.next();
        }

        Ok(Monkey {
            items,
            target,
            op: op.expect("Missing operation"),
            divisible: divisible.expect("Missing divisible"),
            counts_inspections: 0,
        })
    }
}

enum CalmFactor {
    Part1(usize),
    Part2(usize),
}

struct Monkeys {
    monkeys: Vec<Monkey>,
    calm_factor: CalmFactor,
}

impl TryFrom<&str> for Monkeys {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut monkeys = vec![];
        let mut temp = String::new();

        for line in value.lines() {
            if !line.is_empty() {
                temp.push_str(format!("{}\n", line).as_str());
            } else {
                monkeys.push(Monkey::try_from(temp.as_str())?);
                temp.clear();
            }
        }
        monkeys.push(Monkey::try_from(temp.as_str())?);
        temp.clear();

        Ok(Monkeys {
            monkeys,
            calm_factor: CalmFactor::Part1(3),
        })
    }
}

impl Iterator for Monkeys {
    type Item = Vec<usize>;

    fn next(&mut self) -> Option<Self::Item> {
        let len = self.monkeys.len();

        for i in 0..len {
            let mut vec = self.monkeys[i].inspect_and_throw(&self.calm_factor);

            for (target, item) in vec.drain(..) {
                self.monkeys[target].catch(&item);
            }
        }

        Some(
            self.monkeys
                .iter()
                .map(|monkey| monkey.counts_inspections)
                .collect(),
        )
    }
}

fn day11_part1(text: &str) -> usize {
    let monkeys = Monkeys::try_from(text).unwrap();
    monkeys
        .take(20)
        .last()
        .unwrap()
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product()
}

fn day11_part2(text: &str) -> usize {
    let mut monkeys = Monkeys::try_from(text).unwrap();

    let base = monkeys
        .monkeys
        .iter()
        .fold(1, |prod, monkey| prod * monkey.divisible);
    monkeys.calm_factor = CalmFactor::Part2(base);

    monkeys
        .take(10_000)
        .last()
        .unwrap()
        .iter()
        .sorted()
        .rev()
        .take(2)
        .product()
}

pub fn day11(text: &str) {
    println!("day11 part 1: {}", day11_part1(text));
    println!("day11 part 2: {}", day11_part2(text));
}

#[cfg(test)]
mod tests {
    use super::{day11_part1, day11_part2};

    const TEST_INPUT: &str = r#"Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1"#;

    #[test]
    fn fixed_tests1() {
        assert_eq!(day11_part1(TEST_INPUT), 10605);
    }

    #[test]
    fn fixed_tests2() {
        assert_eq!(day11_part2(TEST_INPUT), 2713310158);
    }
}
