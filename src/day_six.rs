use crate::util::get_input;

fn get_start(input: &str, unique_len: usize) -> usize {
    let mut unique_chars = vec![];
    //println!("---");
    for c in input.chars() {
        //println!("{:?}", unique_chars);
        if let Some(pos) = unique_chars.iter().position(|p| p == &c) {
            //    println!("char {} is at pos: {}", c, pos);
            unique_chars.splice(0..pos + 1, vec![]);
        }

        if Some(&c) != unique_chars.last() {
            unique_chars.push(c);
        }

        if unique_chars.len() == unique_len {
            break;
        }
    }

    let s = unique_chars
        .iter()
        .map(|c| c.to_string())
        .collect::<Vec<String>>()
        .join("");

    input.find(&s).unwrap() + s.len()
}

pub fn run() {
    let input = get_input("day_six");
    println!(
        "Part one: First 'start of packet marker' detected at '{}'",
        get_start(&input, 4)
    );
    println!(
        "Part two: First 'start of message marker' detected at '{}'",
        get_start(&input, 14)
    );
}

#[cfg(test)]
mod test {
    use super::get_start;

    #[test]
    fn run_test() {
        assert_eq!(get_start("bvwbjplbgvbhsrlpgdmjqwftvncz", 4), 5);
        assert_eq!(get_start("nppdvjthqldpwncqszvftbrmjlhg", 4), 6);
        assert_eq!(get_start("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4), 10);
        assert_eq!(get_start("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4), 11);
    }
}
