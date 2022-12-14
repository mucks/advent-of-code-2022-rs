use std::{collections::HashMap, hash::Hash};

use crate::util::get_input;

#[derive(Debug, Clone)]
struct Dir {
    name: String,
    path: String,
    parent: Option<Box<Dir>>,
    children: Vec<Dir>,
    size: Option<u32>,
}

impl Dir {
    fn new(parent: Dir, name: &str, path: &str) -> Self {
        Self {
            name: name.to_string(),
            path: path.to_string(),
            parent: Some(Box::new(parent)),
            children: vec![],
            size: None,
        }
    }
    fn initial_root_dir() -> Self {
        Self {
            name: "/".into(),
            path: "/".to_string(),
            parent: None,
            children: vec![],
            size: None,
        }
    }
    fn add_size(&mut self, size: u32) {
        if let Some(s) = &mut self.size {
            *s += size
        } else {
            self.size = Some(size)
        }
    }
    fn add_child(&mut self, child: Dir) {
        self.children.push(child);
    }
    fn add_child_at_path(&mut self, path: &str, new_name: &str) {
        for p in path.split('/') {
            for child in self.children.clone() {
                if child.name == p {
                    if let Some(split) = path.split_once('/') {
                        self.add_child_at_path(split.1, new_name)
                    }
                }
            }
        }
        let mut new_path = "".to_string();
        if path == "/" {
            new_path = format!("{}{}", path, new_name);
        } else {
            new_path = format!("{}/{}", path, new_name);
        }

        let new_dir = Dir::new(self.clone(), new_name, &new_path);
        self.add_child(new_dir);
    }
}

fn move_to_root(dir: &mut Dir) {
    while dir.parent.is_some() {
        *dir = *dir.parent.as_ref().unwrap().to_owned();
    }
}

fn handle_cd(line: &str, dir: &mut Dir, current_path: &mut String) {
    let name = line.replace("$ cd ", "");

    match name.as_str() {
        "/" => {
            *current_path = "/".to_string();
            move_to_root(dir);
        }
        ".." => {
            let mut split = current_path.rsplit('/');
            split.next();
            *current_path = split.collect::<Vec<&str>>().join("");
            if !current_path.contains('/') {
                *current_path = "/".to_string();
            }
            if let Some(parent) = dir.parent.as_ref() {
                *dir = *parent.to_owned();
            }
        }
        _ => {
            // let mut current_child: &mut Dir = dir;

            dir.add_child_at_path(current_path, &name);

            if current_path == &"/".to_string() {
                *current_path = format!("{}{}", current_path, name);
            } else {
                *current_path = format!("{}/{}", current_path, name);
            }
        }
    };
}

fn handle_ls(line: &str, dir: &mut Dir) {
    let split: Vec<&str> = line.split_whitespace().collect();

    if let Some(size_str) = split.first() {
        if let Ok(size) = size_str.parse::<u32>() {
            dir.add_size(size);
        }
    }
}

fn rec(current_dir: &mut Dir) {
    for child in &mut current_dir.children {
        println!("\t{}\t {:?}", child.path, child.size);
        // rec(child)
    }
}

pub fn run() {
    let input = get_input("day_seven_small");
    let mut dir = Dir::initial_root_dir();
    let mut ls_lock = false;
    let mut current_path = String::new();

    for line in input.split('\n') {
        if line.contains("$ cd ") {
            handle_cd(line, &mut dir, &mut current_path);
            ls_lock = false;
        }
        // if ls has been called the line before
        if ls_lock {
            handle_ls(line, &mut dir);
        }

        if line.contains("$ ls") {
            ls_lock = true;
        }
    }

    rec(&mut dir);

    // println!("{:#?}", dirs);
}
