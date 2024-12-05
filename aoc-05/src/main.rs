use std::{num::ParseIntError, str::FromStr};

enum InputError {
    ParseError(String),
}

impl From<ParseIntError> for InputError {
    fn from(_value: ParseIntError) -> Self {
        Self::ParseError("int error".to_owned())
    }
}

#[derive(Debug)]
struct Rule {
    first: u32,
    second: u32,
}

impl FromStr for Rule {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = s.split("|").collect();

        if splitted.len() != 2 {
            return Err(InputError::ParseError("more than two elements".to_owned()));
        }

        let first = u32::from_str(splitted[0])?;
        let second = u32::from_str(splitted[1])?;

        Ok(Rule { first, second })
    }
}

#[derive(Clone, Debug)]
struct QueueItem {
    list: Vec<u32>,
}

impl QueueItem {
    fn valid(&self, rule: &Rule) -> bool {
        match (
            self.list.iter().position(|x| *x == rule.first),
            self.list.iter().position(|x| *x == rule.second),
        ) {
            (Some(first_pos), Some(second_pos)) => first_pos < second_pos,
            _ => true,
        }
    }

    fn valid_all(&self, rules: &Vec<Rule>) -> bool {
        rules.iter().all(|rule| self.valid(rule))
    }

    fn apply(&self, rule: &Rule) -> Option<Self> {
        match (
            self.list.iter().position(|x| *x == rule.first),
            self.list.iter().position(|x| *x == rule.second),
        ) {
            (Some(first_pos), Some(second_pos)) => {
                if first_pos > second_pos {
                    let mut newlist = self.list.clone();
                    newlist[second_pos] = rule.first;
                    newlist[first_pos] = rule.second;
                    Some(Self { list: newlist })
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn middle(&self) -> u32 {
        self.list[self.list.len() / 2]
    }
}

impl FromStr for QueueItem {
    type Err = InputError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let splitted: Vec<_> = s.split(",").collect();
        if splitted.is_empty() {
            return Err(InputError::ParseError("empty".to_owned()));
        }
        let list = splitted
            .into_iter()
            .map(u32::from_str)
            .collect::<Result<Vec<_>, _>>()?;
        Ok(QueueItem { list })
    }
}

fn parse_input(input: &[u8]) -> (Vec<Rule>, Vec<QueueItem>) {
    let input_str = std::str::from_utf8(input).unwrap();

    let mut rules = Vec::<Rule>::new();
    let mut queue = Vec::<QueueItem>::new();

    for line in input_str.lines() {
        if let Ok(rule) = Rule::from_str(line) {
            rules.push(rule);
        }
        if let Ok(queueitem) = QueueItem::from_str(line) {
            queue.push(queueitem);
        }
    }
    (rules, queue)
}

fn get_valids(queue: &Vec<QueueItem>, rules: &Vec<Rule>) -> Vec<QueueItem> {
    let mut valids = Vec::<QueueItem>::new();
    for item in queue {
        if item.valid_all(rules) {
            valids.push(item.clone());
        }
    }
    valids
}

fn get_invalids(queue: &Vec<QueueItem>, rules: &Vec<Rule>) -> Vec<QueueItem> {
    let mut invalids = Vec::<QueueItem>::new();
    for item in queue {
        if !item.valid_all(rules) {
            invalids.push(item.clone());
        }
    }
    invalids
}

fn sum_valids(queue: &Vec<QueueItem>, rules: &Vec<Rule>) -> u32 {
    get_valids(queue, rules).iter().map(|x| x.middle()).sum()
}

fn sum_fixed_invalids(queue: &Vec<QueueItem>, rules: &Vec<Rule>) -> u32 {
    let mut sum = 0;
    println!("{}", get_invalids(queue, rules).len());
    for item in get_invalids(queue, rules) {
        let mut fixed = item;
        for _ in 0..10 {
            for rule in rules.iter() {
                if let Some(output) = fixed.apply(rule) {
                    fixed = output;
                    if fixed.valid_all(rules) {
                        break;
                    }
                }
            }
        }
        if fixed.valid_all(rules) {
            sum += fixed.middle();
        }
    }
    sum
}

fn main() {
    let example = include_bytes!("../example.txt");
    let (example_rules, example_queue) = parse_input(&example[..]);

    println!("example {:?}", sum_valids(&example_queue, &example_rules));

    let input = include_bytes!("../input.txt");
    let (input_rules, input_queue) = parse_input(&input[..]);

    println!("input {:?}", sum_valids(&input_queue, &input_rules));

    println!(
        "fixed invalids {:?}",
        sum_fixed_invalids(&example_queue, &example_rules)
    );
    print!("fixed invalids {:?}", sum_fixed_invalids(&input_queue, &input_rules));
}
