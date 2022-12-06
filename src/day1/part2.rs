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

    let mut elfs = input
        .split("\n\n")
        .map(|items| {
            items
                .lines()
                .map(|calories| calories.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .map(|items| Elf { items })
        .collect::<Vec<Elf>>();

    elfs.sort_by(|elf_a, elf_b| elf_b.total_calories().cmp(&elf_a.total_calories()));

    let elf_a = elfs.get(0).unwrap();
    let elf_b = elfs.get(1).unwrap();
    let elf_c = elfs.get(2).unwrap();
    let sum = elf_a.total_calories() + elf_b.total_calories() + elf_c.total_calories();

    dbg!(
        elf_a,
        elf_b,
        elf_c,
        sum
    );
}
