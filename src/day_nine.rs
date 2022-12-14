use std::collections::HashSet;

use crate::util::get_input;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn print_knots(knots: &Vec<Vector2>) {
    for y in 0..40 {
        println!();
        for x in 0..40 {
            let mut has_printend = false;
            for i in 0..knots.len() {
                if knots[i].y == y && knots[i].x == x {
                    if i == 0 {
                        print!("H")
                    } else {
                        print!("{}", i);
                    }
                    has_printend = true;
                }
            }

            if !has_printend {
                print!(".");
            }
        }
    }
    println!();
    println!();
}

fn print_positions(head_pos: Vector2, tail_pos: Vector2) {
    for y in 0..40 {
        println!();
        for x in 0..40 {
            if y == head_pos.y && x == head_pos.x {
                print!("H");
            } else if y == tail_pos.y && x == tail_pos.x {
                print!("T");
            } else if x == 0 && y == 5 {
                print!("s");
            } else {
                print!(".");
            }
        }
    }
    println!();
    println!();
}

fn insert_tail_pos(
    set: &mut HashSet<Vector2>,
    tail_pos: Vector2,
    tail_index: usize,
    tail_size: usize,
) {
    if !set.contains(&tail_pos) && tail_index == tail_size - 1 {
        set.insert(tail_pos);
    }
}

fn apply_direction(
    knots: &mut Vec<Vector2>,
    step: i32,
    direction: &str,
    set: &mut HashSet<Vector2>,
) {
    let head = knots[0].clone();
    let (vertical, add, range): (_, _, Vec<i32>) = match direction {
        "R" => (false, 1, (head.x..head.x + step).collect()),
        "L" => (false, -1, (head.x - step..head.x).rev().collect()),
        "D" => (true, 1, (head.y..head.y + step).collect()),
        "U" => (true, -1, (head.y - step..head.y).rev().collect()),
        _ => panic!("invalid direction: {}", direction),
    };

    for _ in range {
        for i in 0..knots.len() - 1 {
            let tail_index = i + 1;
            let mut head = knots[i];
            let mut tail = knots[tail_index];

            let (head_i, head_j) = if vertical {
                (&mut head.y, &mut head.x)
            } else {
                (&mut head.x, &mut head.y)
            };
            let (tail_i, tail_j) = if vertical {
                (&mut tail.y, &mut tail.x)
            } else {
                (&mut tail.x, &mut tail.y)
            };

            *head_i += add;
            if head_i.abs_diff(*tail_i) > 1 {
                if head_j.abs_diff(*tail_j) > 0 {
                    *tail_j = *head_j;
                }
                *tail_i += add;
                insert_tail_pos(set, tail, tail_index, knots.len());
            }

            knots[i] = head;
            knots[tail_index] = tail;
        }
    }
}

fn follow_tail(instruction: &str, knots: &mut Vec<Vector2>, set: &mut HashSet<Vector2>) {
    // print_positions(head_pos, tail_pos);
    // println!("{}", instruction);
    let split: Vec<&str> = instruction.split_whitespace().collect();
    let direction = split[0];
    let step: i32 = split[1].parse().unwrap();

    apply_direction(knots, step, direction, set);
}

pub fn run() {
    let input = get_input("day_nine");

    let mut knots: Vec<Vector2> = vec![];

    // part one '2' knots
    // part two '10' knots
    for _i in 0..2 {
        knots.push(Vector2::new(0, 0));
    }

    let mut set: HashSet<Vector2> = HashSet::new();
    set.insert(knots[1]);
    // add initial position

    for instruction in input.split('\n') {
        follow_tail(instruction, &mut knots, &mut set);
    }

    println!(
        "The tail of the rope visited '{}' positions at least once",
        set.len()
    );
}
