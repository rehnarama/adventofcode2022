#[derive(Debug, Eq, PartialEq, Clone, Copy)]
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

fn main() {
    let input = include_str!("input.txt");

    let priority_sum: u32 = input
        .lines()
        .map(|line| Rugsack::from(line))
        .map(|rugsack| rugsack.find_misplaced().clone())
        .map(|misplaced| misplaced.priority())
        .sum();

    dbg!(priority_sum);
}
