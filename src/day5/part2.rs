use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Crate {
    label: String,
}

impl FromStr for Crate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"\[([A-Z])\]").unwrap();
        }

        let captures = re.captures(s).ok_or("Couldn't parse crate")?;
        assert!(captures.len() == 2);

        let crate_label = captures.get(1).ok_or("Couldn't parse crate")?;
        Ok(Crate {
            label: crate_label.as_str().to_string(),
        })
    }
}

#[derive(Debug)]
struct Stacks {
    stacks: Vec<Vec<Crate>>,
}

impl FromStr for Stacks {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut stacks = vec![];

        let first_line = s.lines().find(|_| true).unwrap();
        // 3 characters + 1 space per box, except
        let total_columns = (first_line.len() + 1) / 4;
        for i in 0..total_columns {
            stacks.push(vec![]);
        }

        let rows: Vec<Vec<Option<Crate>>> = s
            .lines()
            .rev()
            .skip(1)
            .map(|line| {
                let mut row: Vec<Option<Crate>> = vec![];
                for i in 0..total_columns {
                    let parsed = &line[(i * 4)..(i * 4 + 3)].parse::<Crate>();
                    let cell = match parsed {
                        Ok(cell) => Some(cell.clone()),
                        Err(_) => None,
                    };

                    row.push(cell);
                }

                row
            })
            .collect();

        for row in rows {
            let mut i = 0;
            for column in row {
                let stack = stacks.get_mut(i).unwrap();

                if let Some(cell) = column {
                    stack.push(cell);
                }

                i += 1;
            }
        }

        Ok(Stacks { stacks })
    }
}

#[derive(Debug)]
struct Move {
    amount: usize,
    from: usize,
    to: usize,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref re: Regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        }

        let captures = re.captures(s).unwrap();

        let amount = captures.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = captures.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = captures.get(3).unwrap().as_str().parse::<usize>().unwrap();

        Ok(Move { amount, from, to })
    }
}

fn main() -> Result<(), String> {
    let input = include_str!("input.txt").replace("\r", "");

    let (state, moves) = input.split_once("\n\n").unwrap();

    let mut stacks = state.parse::<Stacks>().unwrap();
    let moves: Vec<Move> = moves
        .lines()
        .map(|line| line.parse::<Move>().unwrap())
        .collect();

    for m in moves {
        let target_stack_size = stacks.stacks.get(m.to - 1).unwrap().len();

        for i in 0..m.amount {
            let c = stacks.stacks.get_mut(m.from - 1).unwrap().pop().unwrap();
            stacks.stacks.get_mut(m.to - 1).unwrap().insert(target_stack_size, c)
        }
    }

    let result = stacks
        .stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .map(|c| c.label.clone())
        .collect::<String>();

    dbg!(result);

    Ok(())
}
