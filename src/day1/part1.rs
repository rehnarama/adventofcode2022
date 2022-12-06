#[derive(Debug)]
struct Elf {
    items: Vec<usize>,
}

impl Elf {
    fn total_calories(&self) -> usize {
        self.items.iter().sum()
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let elfs = input
        .split("\n\n")
        .map(|items| {
            items
                .lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|items| Elf { items })
        .collect::<Vec<Elf>>();

    let biggest_elf = elfs
        .iter()
        .max_by(|elf_a, elf_b| elf_a.total_calories().cmp(&elf_b.total_calories()));

    dbg!(biggest_elf.unwrap().total_calories());
}
