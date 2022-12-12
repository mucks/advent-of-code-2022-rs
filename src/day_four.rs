use crate::util::get_input;

fn get_elf_range(elf_str: &str) -> Vec<u32> {
    let split: Vec<&str> = elf_str.split('-').collect();
    let start: u32 = split[0].parse().unwrap();
    let end: u32 = split[1].parse().unwrap();

    let mut range = vec![];
    for i in start..end + 1 {
        range.push(i)
    }
    range
}

pub fn run() {
    let input = get_input("day_four");

    let mut fully_contains_count = 0;
    let mut single_contains_count = 0;

    for line in input.split('\n') {
        let split: Vec<&str> = line.split(',').collect();
        let range_one = get_elf_range(split[0]);
        let range_two = get_elf_range(split[1]);

        if range_one.iter().all(|r| range_two.contains(r))
            || range_two.iter().all(|r| range_one.contains(r))
        {
            fully_contains_count += 1;
        }

        if range_one.iter().any(|r| range_two.contains(r))
            || range_two.iter().any(|r| range_one.contains(r))
        {
            single_contains_count += 1;
        }
    }

    println!(
        "{} pairs are fully contained within the other",
        fully_contains_count
    );
    println!("{} pairs have an overlap", single_contains_count);
}
