use crate::util::get_input;

static ASCII_LOWER: [char; 26] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z',
];

static ASCII_UPPER: [char; 26] = [
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

fn get_char_value(ch: &char) -> u32 {
    if let Some(i) = ASCII_LOWER.iter().position(|c| c == ch) {
        return i as u32 + 1;
    }
    if let Some(i) = ASCII_UPPER.iter().position(|c| c == ch) {
        return i as u32 + 27;
    }
    0
}

fn get_priority(line: &str) -> u32 {
    let half_len = line.len() / 2;
    let first_compartment = &line[0..half_len];
    let second_compartment = &line[half_len..line.len()];

    let fc_chars: Vec<char> = first_compartment.chars().collect();
    let sc_chars: Vec<char> = second_compartment.chars().collect();

    for fc_char in fc_chars {
        if let Some(sc_char) = sc_chars.iter().find(|c| c == &&fc_char) {
            return get_char_value(sc_char);
        }
    }
    0
}

fn get_priority_part_two(lines: &[&str]) -> u32 {
    for first_c in lines[0].chars() {
        if lines[1].chars().any(|c| first_c == c) && lines[2].chars().any(|c| first_c == c) {
            return get_char_value(&first_c);
        }
    }

    0
}
pub fn run() {
    let input = get_input("day_three");
    let mut priorities = vec![];
    for line in input.split('\n') {
        priorities.push(get_priority(line));
    }
    println!(
        "Part one: Priority Sum is: {}",
        priorities.iter().sum::<u32>()
    );

    priorities = vec![];
    for lines in input.split('\n').collect::<Vec<&str>>().chunks(3) {
        priorities.push(get_priority_part_two(lines))
    }
    println!(
        "Part two: Priority Sum is: {}",
        priorities.iter().sum::<u32>()
    );
}
