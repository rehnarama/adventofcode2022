use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Clone, Copy, Hash)]
struct Item {
    item: char,
}

impl From<char> for Item {
    fn from(c: char) -> Self {
        Item { item: c }
    }
}

impl Item {
    fn priority(&self) -> u32 {
        if self.item >= 'a' && self.item <= 'z' {
            (self.item as u32) - ('a' as u32) + 1
        } else {
            (self.item as u32) - ('A' as u32) + 27
        }
    }
}

struct Rugsack {
    compartments: (Vec<Item>, Vec<Item>),
}

impl Rugsack {
    fn iter(&self) -> std::iter::Chain<std::slice::Iter<Item>, std::slice::Iter<Item>> {
        self.compartments.0.iter().chain(self.compartments.1.iter())
    }
}

impl From<&str> for Rugsack {
    fn from(s: &str) -> Self {
        let mut compartments = (vec![], vec![]);

        let compartment_size = s.len() / 2;

        s.chars().enumerate().for_each(|(index, item)| {
            let &mut compartment;

            if index >= compartment_size {
                compartment = &mut compartments.0;
            } else {
                compartment = &mut compartments.1;
            }

            compartment.push(item.into());
        });

        Rugsack { compartments }
    }
}

impl Rugsack {
    fn find_misplaced(&self) -> &Item {
        self.compartments
            .0
            .iter()
            .find(|item| {
                self.compartments
                    .1
                    .iter()
                    .any(|other_item| item == &other_item)
            })
            .unwrap()
    }
}

fn find_common(elf_a: &Rugsack, elf_b: &Rugsack, elf_c: &Rugsack) -> Item {
    let items_a: HashSet<&Item> = HashSet::from_iter(elf_a.iter());
    let items_b: HashSet<&Item> = HashSet::from_iter(elf_b.iter());
    let items_c: HashSet<&Item> = HashSet::from_iter(elf_c.iter());

    let common_ab: HashSet<&Item> = HashSet::from_iter(items_a.intersection(&items_b).map(|r| *r));
    let mut common_abc = common_ab.intersection(&items_c).map(|r| *r);

    common_abc.find(|_| true).unwrap().clone()
}

fn main() {
    let input = include_str!("input.txt");

    let rugsacks: Vec<Rugsack> = input.lines().map(|line| Rugsack::from(line)).collect();

    let mut commons = vec![];

    for i in (0..rugsacks.len()).step_by(3) {
        let elf_a = rugsacks.get(i + 0).unwrap();
        let elf_b = rugsacks.get(i + 1).unwrap();
        let elf_c = rugsacks.get(i + 2).unwrap();

        let common = find_common(elf_a, elf_b, elf_c);
        commons.push(common);
    }

    let priority_sum: u32 = commons.iter()
        .map(|i| i.priority())
        .sum();

    dbg!(priority_sum);
}
