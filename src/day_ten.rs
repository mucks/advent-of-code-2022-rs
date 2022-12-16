use crate::util::get_input;

pub fn run() {
    let input = get_input("day_ten");
    let instructions: Vec<&str> = input.split('\n').collect();

    let mut x_register = 1;
    let mut instruction_index = 0;
    //<part-one>
    let mut total_signal_strength = 0;
    let mut wait_for_addx = false;
    //</part-one>
    //<part-two>
    let mut crt_rows: Vec<Vec<char>> = vec![];
    let mut crt_row: Vec<char> = vec![];
    let mut crt_position = 0;
    //</part-two>

    for cycle in 1..instructions.len() as i32 * 2 {
        //<part-one>
        if cycle == 20 || (cycle + 20) % 40 == 0 {
            println!("cycle: {}, x_registar: {}", cycle, x_register);
            let signal_strength = cycle * x_register;
            total_signal_strength += signal_strength;
        }
        //</part-one>
        //<part-two>
        if crt_row.len() >= 40 {
            crt_rows.push(crt_row);
            crt_row = vec![];
            crt_position = 0;
        }
        if crt_position <= x_register + 1 && crt_position >= x_register - 1 {
            crt_row.push('#');
        } else {
            crt_row.push('.');
        }
        //</part-two>
        let instruction = match instructions.get(instruction_index) {
            Some(i) => i,
            None => {
                break;
            }
        };

        // If waited one cycle for addx
        if wait_for_addx {
            let split = instruction.split_once(' ').expect("invalid instruction!");
            let v: i32 = split.1.parse().unwrap();
            x_register += v;
            wait_for_addx = false;
            instruction_index += 1;
        } else if instruction == &"noop" {
            // IF noop is called just increase the instruction index
            instruction_index += 1;
        } else {
            // IF addx is called wait 1 cycle
            wait_for_addx = true;
        }
        //part-two
        crt_position += 1;
    }

    println!(
        "\nPart One:\ntotal signal strength: {}",
        total_signal_strength
    );
    println!("\nPart Two:\n");
    for row in crt_rows {
        println!("{}", row.iter().collect::<String>());
    }
}
