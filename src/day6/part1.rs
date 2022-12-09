use std::{collections::HashSet};

fn is_unique_chars(s: &str) -> bool {
    let set: HashSet<char> = HashSet::from_iter(s.chars());

    s.len() == set.len()
}


fn main() {
    let input = include_str!("input.txt");

    for i in 0..(input.len() - 4) {
        let chars = &input[i..(i+4)];
        if is_unique_chars(chars) {
            println!("Start sequence found at {}", i + 4);
            break;
        }

    }
}