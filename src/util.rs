use std::{fs, io::Read};

pub fn get_input(day_name: &str) -> String {
    let mut f = fs::File::open(&format!("./input/{}.txt", day_name)).unwrap();
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    s
}
