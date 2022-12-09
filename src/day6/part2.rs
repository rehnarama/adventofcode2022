use std::collections::HashSet;

fn is_unique_chars(s: &str) -> bool {
    let set: HashSet<char> = HashSet::from_iter(s.chars());

    s.len() == set.len()
}

fn main() {
    let input = include_str!("input.txt");

    let n_distinct = 14;

    for i in 0..(input.len() - n_distinct) {
        let chars = &input[i..(i + n_distinct)];
        if is_unique_chars(chars) {
            println!("Start sequence found at {}", i + n_distinct);
            break;
        }
    }
}
