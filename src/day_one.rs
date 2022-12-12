use crate::util::get_input;

pub fn run() {
    let input = get_input("day_one");
    let elf_calorie_value_strings: Vec<&str> = input.split("\n\n").collect();

    let mut elf_calorie_sums = Vec::new();

    for elf_calorie_value_string in elf_calorie_value_strings {
        // println!("---\n{}\n---", elf_calorie_value_string);
        let elf_calorie_sum: u32 = elf_calorie_value_string
            .split('\n')
            .filter_map(|s| s.parse::<u32>().ok())
            .sum();
        elf_calorie_sums.push(elf_calorie_sum);
    }

    println!(
        "Calories carried by the elf with the most calories: {}",
        elf_calorie_sums.iter().max().unwrap()
    );

    elf_calorie_sums.sort_by(|a, b| b.cmp(a));
    println!(
        "Calories carried by the top 3 elves with the most calories: {}",
        elf_calorie_sums[0..3].iter().sum::<u32>()
    );
}
