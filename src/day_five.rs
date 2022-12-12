use std::collections::{BTreeMap, VecDeque};

use crate::util::get_input;

fn print_stacks(stacks: &BTreeMap<u32, Vec<char>>) {
    println!();

    let max_len = stacks
        .values()
        .max_by(|a, b| a.len().cmp(&b.len()))
        .unwrap()
        .len();

    for j in 0..max_len {
        for i in 0..stacks.len() {
            let index = (i + 1) as u32;
            let stack = stacks.get(&index).unwrap();

            if let Some(c) = stack.get(j) {
                print!(" [{}] ", c)
            } else {
                print!("     ");
            }
        }
        println!();
    }
    for i in 0..stacks.len() {
        print!("  {}  ", i + 1)
    }
    println!();
}

pub fn run() {
    let input = get_input("day_five");

    let split = input.split("\n\n").collect::<Vec<&str>>();

    let state = split[0];

    let mut stacks: BTreeMap<u32, Vec<char>> = BTreeMap::new();

    for line in state.split('\n') {
        let mut stack_index = 1;
        let line_chars: Vec<char> = line.chars().collect();
        for i in (0..line_chars.len()).step_by(4) {
            let val = &line_chars[i + 1];
            stacks.entry(stack_index).or_insert_with(Vec::new);
            let stack = stacks.get_mut(&stack_index).unwrap();
            if val.is_uppercase() {
                stack.push(*val);
            }
            stack_index += 1;
        }
    }

    let instructions = split[1];

    for instruction in instructions.split('\n') {
        print_stacks(&stacks);
        let split: Vec<&str> = instruction.split_whitespace().collect();

        let amount: u32 = split[1].parse().unwrap();
        let from: u32 = split[3].parse().unwrap();
        let to: u32 = split[5].parse().unwrap();

        // part 1
        // for _ in 0..amount {
        //     // remove crate from top of the 'from' stack
        //     let from_stack = stacks.get_mut(&from).unwrap();
        //     let c = from_stack.remove(0);

        //     // put crate on top of the 'to' stack
        //     let to_stack = stacks.get(&to).unwrap();
        //     let mut new_to_stack = vec![c];
        //     new_to_stack.extend(to_stack);
        //     let to_stack = stacks.get_mut(&to).unwrap();
        //     *to_stack = new_to_stack;
        // }

        //part 2
        let mut crate_storage: Vec<char> = vec![];
        for _ in 0..amount {
            let from_stack = stacks.get_mut(&from).unwrap();
            let c = from_stack.remove(0);
            crate_storage.push(c);
        }
        let to_stack = stacks.get(&to).unwrap();
        crate_storage.extend(to_stack);
        let to_stack = stacks.get_mut(&to).unwrap();
        *to_stack = crate_storage;
    }

    let mut top_chars = vec![];
    for (_, v) in stacks {
        top_chars.push(v[0].to_string());
    }

    let result = top_chars.join("");
    println!("The crates on top are: {}", result);
}
