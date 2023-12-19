use std::collections::HashMap;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Range {
    start: usize,
    end: usize,
}

impl Range {
    fn new() -> Self {
        Self {
            start: 1,
            end: 4000,
        }
    }

    fn len(&self) -> usize {
        self.end - self.start + 1
    }

    fn less_than(&mut self, value: usize) {
        if self.end > value {
            self.end = value - 1;
        }
    }

    fn greater_than(&mut self, value: usize) {
        if self.start < value {
            self.start = value + 1;
        }
    }
}

#[derive(Debug, Clone)]
struct MachinePart {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl MachinePart {
    fn new(x: usize, m: usize, a: usize, s: usize) -> Self {
        Self { x, m, a, s }
    }

    fn parse_machine_parts(input: &str) -> Self {
        let input = input.split_once("{").unwrap().1;
        let input = (&input[..input.len() - 1])
            .split(",")
            .collect::<Vec<&str>>();

        let (mut x, mut m, mut a, mut s) = (0, 0, 0, 0);
        for i in 0..input.len() {
            let tmp = input[i].split_once("=").unwrap();
            let tmp2 = tmp.1.parse::<usize>().unwrap();

            match tmp.0 {
                "x" => x = tmp2,
                "m" => m = tmp2,
                "a" => a = tmp2,
                "s" => s = tmp2,
                _ => {}
            }
        }

        Self::new(x, m, a, s)
    }

    fn sum_features(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}

#[derive(Debug, Clone)]
struct MachinePartRanges {
    x: Range,
    m: Range,
    a: Range,
    s: Range,
}

impl MachinePartRanges {
    fn new() -> Self {
        Self {
            x: Range::new(),
            m: Range::new(),
            a: Range::new(),
            s: Range::new(),
        }
    }

    fn product(&self) -> usize {
        self.x.len() * self.m.len() * self.a.len() * self.s.len()
    }

    fn is_in_range(&self, value: &MachinePart) -> bool {
        self.x.start <= value.x
            && self.x.end >= value.x
            && self.m.start <= value.m
            && self.m.end >= value.m
            && self.a.start <= value.a
            && self.a.end >= value.a
            && self.s.start <= value.s
            && self.s.end >= value.s
    }
}

#[derive(Debug, Clone)]
enum Feature {
    X,
    M,
    A,
    S,
}

impl FromStr for Feature {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "x" => Ok(Self::X),
            "m" => Ok(Self::M),
            "a" => Ok(Self::A),
            "s" => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    LessThan(usize),
    GreaterThan(usize),
}

impl FromStr for Operator {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let value = s[1..].parse::<usize>().unwrap();
        match &s[..1] {
            "<" => Ok(Self::LessThan(value)),
            ">" => Ok(Self::GreaterThan(value)),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
enum Action {
    Accept,
    Reject,
    Jump(String),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            s => Ok(Self::Jump(s.to_string())),
        }
    }
}

type Term = (Feature, Operator, Action);

#[derive(Debug, Clone)]
struct Rule {
    name: String,
    constraints: Vec<Term>,
    else_: Action,
}

impl Rule {
    fn new(name: &str, constraints: Vec<Term>, else_: Action) -> Self {
        Self {
            name: name.to_string(),
            constraints,
            else_,
        }
    }
}

impl FromStr for Rule {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut constraints = Vec::new();

        let s = s.split_once("{").unwrap();
        let name = s.0;
        let s = s.1;
        let mut split_terms = (&s[..s.len() - 1]).split(",").collect::<Vec<&str>>();

        for rule in &split_terms {
            if let Some(rule) = rule.split_once(":") {
                let feature = rule.0[0..1].parse::<Feature>().unwrap();
                let operator = rule.0[1..].parse::<Operator>().unwrap();
                let action = rule.1.parse::<Action>().unwrap();

                constraints.push((feature, operator, action));
            }
        }

        let else_ = split_terms.pop().unwrap().parse::<Action>().unwrap();

        Ok(Self::new(&name, constraints, else_))
    }
}

struct Workflows {
    rules: HashMap<String, Rule>,
    machine_parts: Vec<MachinePart>,
}

impl Workflows {
    fn sum_accepted(&self) -> usize {
        self.machine_parts
            .iter()
            .filter(|&machine_part| self.is_accepted(machine_part))
            .map(|machine_part| machine_part.sum_features())
            .sum()
    }

    fn is_accepted(&self, machine_part: &MachinePart) -> bool {
        let mut current_rule = self.rules.get("in").unwrap();
        let mut result = None;

        'res: while result.is_none() {
            for constraint in &current_rule.constraints {
                let (f, o, a) = constraint;
                let value = match f {
                    Feature::X => machine_part.x,
                    Feature::M => machine_part.m,
                    Feature::A => machine_part.a,
                    Feature::S => machine_part.s,
                };

                let comparison_result = match o {
                    Operator::LessThan(v) => value < *v,
                    Operator::GreaterThan(v) => value > *v,
                };

                if comparison_result {
                    match a {
                        Action::Jump(rule_name) => {
                            current_rule = self.rules.get(rule_name).unwrap();
                            continue 'res;
                        }
                        v => {
                            result = Some(v);
                            break 'res;
                        }
                    }
                }
            }

            match &current_rule.else_ {
                Action::Jump(rule_name) => {
                    current_rule = self.rules.get(rule_name).unwrap();
                }
                v => {
                    result = Some(v);
                }
            }
        }

        *result.unwrap() == Action::Accept
    }

    fn apply_constraint(
        &self,
        rule: &Rule,
        mut range: MachinePartRanges,
    ) -> Vec<MachinePartRanges> {
        let mut result = Vec::new();

        for constraint in &rule.constraints {
            let mut current_range = range.clone();
            let (f, o, a) = &constraint;

            let (mut value, mut value_for_top_range) = match &f {
                Feature::X => (&mut current_range.x, &mut range.x),
                Feature::M => (&mut current_range.m, &mut range.m),
                Feature::A => (&mut current_range.a, &mut range.a),
                Feature::S => (&mut current_range.s, &mut range.s),
            };

            match o {
                Operator::LessThan(v) => {
                    value.less_than(*v);
                    value_for_top_range.greater_than(*v - 1);
                }
                Operator::GreaterThan(v) => {
                    value.greater_than(*v);
                    value_for_top_range.less_than(*v + 1);
                }
            }

            match a {
                Action::Jump(rule_name) => {
                    let rule = self.rules.get(rule_name).unwrap();
                    let mut new_ranges = self.apply_constraint(rule, current_range);
                    result.append(&mut new_ranges);
                }
                Action::Accept => {
                    result.push(current_range);
                }
                Action::Reject => continue,
            }
        }

        match &rule.else_ {
            Action::Jump(rule_name) => {
                let rule = self.rules.get(rule_name).unwrap();
                let mut new_ranges = self.apply_constraint(rule, range);
                result.append(&mut new_ranges);
            }
            Action::Accept => {
                result.push(range);
            }
            Action::Reject => {}
        }

        result
    }

    fn get_ranges(&self) -> Vec<MachinePartRanges> {
        let rule = self.rules.get("in").unwrap();
        let range = MachinePartRanges::new();
        self.apply_constraint(rule, range)
    }
    fn count_acceptance_ranges(&self) -> usize {
        self.get_ranges().iter().map(|r| r.product()).sum()
    }

    fn is_machine_part_accepted(&self, part: &MachinePart) -> bool {
        self.get_ranges().iter().any(|r| r.is_in_range(part))
    }

    /// Alternative to part 1 naive implementation
    fn sum_accepted2(&self) -> usize {
        self.machine_parts
            .iter()
            .filter(|&machine_part| self.is_machine_part_accepted(machine_part))
            .map(|machine_part| machine_part.sum_features())
            .sum()
    }
}

impl FromStr for Workflows {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut machine_parts = Vec::new();
        let mut rules = HashMap::new();
        let mut rules_done = false;

        for line in s.lines() {
            if line == "" {
                rules_done = true;
                continue;
            }

            match rules_done {
                false => {
                    let rule = line.parse::<Rule>().unwrap();
                    rules.insert(rule.name.to_string(), rule);
                }
                true => {
                    let machine_part = MachinePart::parse_machine_parts(line);
                    machine_parts.push(machine_part);
                }
            }
        }

        Ok(Self {
            rules,
            machine_parts,
        })
    }
}

fn part1(input: &str) -> usize {
    let workflows = input.parse::<Workflows>().unwrap();
    //workflows.sum_accepted()
    workflows.sum_accepted2()
}

fn part2(input: &str) -> usize {
    let workflows = input.parse::<Workflows>().unwrap();
    workflows.count_acceptance_ranges()
}

pub fn day() -> String {
    let input = include_str!("../input/day19.txt");
    format!("Day 19\tPart 1: {}\tPart 2: {}", part1(input), part2(input))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}"#;

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 19114);
    }

    #[test]
    fn test_part2() {
        let tmp = part2(INPUT);
        let expected = 167409079868000;
        assert_eq!(
            tmp,
            expected,
            "Diff: {}",
            (tmp.max(expected) - tmp.min(expected))
        );
    }
}
