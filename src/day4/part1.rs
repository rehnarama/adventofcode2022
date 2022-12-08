use std::ops::Range;

fn to_range(s: &str) -> Range<usize> {
    let (start_str, end_str) = s.split_once('-').unwrap();
    
    let start: usize = start_str.parse().unwrap();
    let end: usize = end_str.parse().unwrap();
    
    Range { start, end: end + 1 }
}

fn is_contained(r1: &Range<usize>, r2: &Range<usize>) -> bool {
    let contained_in_r1 = r1.contains(&r2.start) && r1.contains(&(r2.end - 1));
    let contained_in_r2 = r2.contains(&r1.start) && r2.contains(&(r1.end - 1));

    contained_in_r1 || contained_in_r2
}

fn main() {
    let input = include_str!("input.txt");

    let number_contained = input.lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(first_range, second_range)| (to_range(first_range), to_range(second_range)))
        .filter(|(r1, r2)| is_contained(r1, r2))
        .count();

    dbg!(number_contained);
}
