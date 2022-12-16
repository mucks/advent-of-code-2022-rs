use std::collections::{BTreeMap, VecDeque};

use crate::util::get_input;

const PART_TWO: bool = true;

#[derive(Debug, Clone)]
struct Monkey {
    id: u64,
    items: VecDeque<u64>,
    operation: String,
    divisible_by: u64,
    true_condition_monkey_id: u64,
    false_condition_monkey_id: u64,
    inspection_count: u64,
}

impl Monkey {
    fn from_monkey_str(monkey_str: &str) -> Self {
        let lines: Vec<&str> = monkey_str.split('\n').collect();
        let id: u64 = lines[0]
            .replace("Monkey ", "")
            .replace(":", "")
            .parse()
            .unwrap();

        let items: VecDeque<u64> = lines[1]
            .replace("Starting items: ", "")
            .trim()
            .split(",")
            .map(|s| s.trim().parse().unwrap())
            .collect();

        let operation = lines[2].replace("Operation: new = old ", "");

        let divisible_by = lines[3]
            .replace("Test: divisible by ", "")
            .trim()
            .parse()
            .unwrap();

        let true_condition_monkey_id = lines[4]
            .replace("If true: throw to monkey ", "")
            .trim()
            .parse()
            .unwrap();
        let false_condition_monkey_id = lines[5]
            .replace("If false: throw to monkey ", "")
            .trim()
            .parse()
            .unwrap();

        Self {
            id,
            items,
            operation,
            divisible_by,
            false_condition_monkey_id,
            true_condition_monkey_id,
            inspection_count: 0,
        }
    }

    fn inspect(&mut self, lcd: u64) -> (u64, u64) {
        let item = self.items.pop_front().unwrap();
        let new_item = self.apply_operation(item, lcd);
        let target = self.get_throw_target(new_item);
        self.inspection_count += 1;

        (target, new_item)
    }

    fn apply_operation(&self, mut item: u64, lcd: u64) -> u64 {
        let split: Vec<&str> = self.operation.split_whitespace().collect();
        let op = split[0];
        let val: u64 = split[1].trim().parse().unwrap_or(item);
        if op.contains('+') {
            item += val;
        } else {
            item *= val;
        }

        if PART_TWO {
            //part two
            item % lcd
        } else {
            //part one
            (item as f64 / 3_f64).floor() as u64
        }
    }

    fn get_throw_target(&self, item: u64) -> u64 {
        if item % self.divisible_by == 0 {
            self.true_condition_monkey_id
        } else {
            self.false_condition_monkey_id
        }
    }

    fn add_item(&mut self, item: u64) {
        self.items.push_back(item);
    }
}

pub fn run() {
    let input = get_input("day_eleven");
    let mut monkeys: BTreeMap<u64, Monkey> = BTreeMap::new();

    for monkey_str in input.split("\n\n") {
        let monkey = Monkey::from_monkey_str(monkey_str);
        monkeys.insert(monkey.id, monkey);
    }

    let round_amount = 10000;

    let keys: Vec<u64> = monkeys.keys().copied().collect();
    let lcd: u64 = monkeys.values().map(|m| m.divisible_by).product();

    for round in 0..round_amount {
        for k in &keys {
            for _ in 0..monkeys.get(k).unwrap().items.len() {
                let (target, item) = {
                    let monkey = monkeys.get_mut(k).unwrap();
                    monkey.inspect(lcd)
                };

                if let Some(monkey) = monkeys.get_mut(&target) {
                    monkey.add_item(item);
                }
            }
        }
    }

    let mut inspection_counts = vec![];

    for (_, monkey) in &monkeys {
        inspection_counts.push(monkey.inspection_count);
        println!(
            "Monkey {} inspected items {} times.",
            monkey.id, monkey.inspection_count
        );
    }

    inspection_counts.sort();
    let last_index = inspection_counts.len() - 1;
    let monkey_business = inspection_counts[last_index] * inspection_counts[last_index - 1];

    println!("\nMonkey Business: {}\n", monkey_business);
}
