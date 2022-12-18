use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug)]
enum Operation {
    Divide(i32),
    Add(i32),
    Multiply(i32),
    AddSelf,
    MultiplySelf,
}

impl From<(&str, &str)> for Operation {
    fn from(args: (&str, &str)) -> Self {
        let arg = args.1.parse::<i32>();

        match args.0 {
            "*" => {
                if arg.is_ok() {
                    Operation::Multiply(arg.unwrap())
                } else {
                    Operation::MultiplySelf
                }
            }
            "+" => {
                if arg.is_ok() {
                    Operation::Add(arg.unwrap())
                } else {
                    Operation::AddSelf
                }
            }
            "/" => Operation::Divide(arg.unwrap()),
            _ => panic!("Unknown operation"),
        }
    }
}

#[derive(Debug)]
struct Item {
    worry_level: i32,
}

impl Item {
    fn apply_op(&mut self, operation: &Operation) {
        self.worry_level = match operation {
            Operation::Divide(arg) => self.worry_level / arg,
            Operation::Add(arg) => self.worry_level + arg,
            Operation::Multiply(arg) => self.worry_level * arg,
            Operation::AddSelf => self.worry_level + self.worry_level,
            Operation::MultiplySelf => self.worry_level * self.worry_level,
        };
    }

    fn test_op(&self, operation: &Operation) -> bool {
        match operation {
            Operation::Divide(arg) => self.worry_level % arg == 0,
            _ => panic!("Can't test other than divisible op"),
        }
    }
}

#[derive(Debug)]
struct Monkey {
    name: usize,
    items: Vec<Item>,
    operation: Operation,
    test: Operation,
    truthy_target: usize,
    falsey_target: usize,
    n_inspections: usize,
}

impl FromStr for Monkey {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MonkeyRE: Regex = Regex::new(r"(\d+)").unwrap();
            static ref ItemsRE: Regex = Regex::new(r"(\d+)").unwrap();
            static ref OperationRE: Regex = Regex::new(r"([+*])\s?(\d+|old)").unwrap();
            static ref TestRE: Regex = Regex::new(r"(\d+)").unwrap();
            static ref TargetRE: Regex = Regex::new(r"(\d+)").unwrap();
        }

        let lines = s.lines().collect::<Vec<&str>>();
        let monkey_s = lines.get(0).ok_or("Couldn't get monkey line")?;
        let items_s = lines.get(1).ok_or("Couldn't get item line")?;
        let operation_s = lines.get(2).ok_or("Couldn't get operation line")?;
        let test_s = lines.get(3).ok_or("Couldn't get test line")?;
        let truthy_target_s = lines.get(4).ok_or("Couldn't get truthy target line")?;
        let falsey_target_s = lines.get(5).ok_or("Couldn't get falsey target line")?;

        let name = MonkeyRE
            .captures(monkey_s)
            .map(|c| c.get(0).unwrap())
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let items: Vec<Item> = ItemsRE
            .captures_iter(items_s)
            .map(|c| c.get(0).unwrap().as_str())
            .map(|s| s.parse::<i32>().unwrap())
            .map(|i| Item { worry_level: i })
            .collect();

        let operation: Operation = OperationRE
            .captures(operation_s)
            .map(|c| (c.get(1).unwrap().as_str(), c.get(2).unwrap().as_str()))
            .map(|(op, arg)| Operation::from((op, arg)))
            .unwrap();

        let test: Operation = TargetRE
            .captures(test_s)
            .map(|c| c.get(0).unwrap())
            .map(|s| s.as_str().parse::<i32>().unwrap())
            .map(|t| Operation::Divide(t))
            .unwrap();

        let truthy_target = TargetRE
            .captures(truthy_target_s)
            .map(|c| c.get(0).unwrap())
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .unwrap();

        let falsey_target = TargetRE
            .captures(falsey_target_s)
            .map(|c| c.get(0).unwrap())
            .map(|m| m.as_str().parse::<usize>().unwrap())
            .unwrap();

        let monkey = Monkey {
            name,
            items,
            operation,
            test,
            truthy_target,
            falsey_target,
            n_inspections: 0,
        };

        Ok(monkey)
    }
}

impl Monkey {
    fn do_turn(&mut self) -> Option<(usize, Item)> {
        if (self.items.len() == 0) {
            return None;
        }

        let mut item = self.items.remove(0);
        self.n_inspections = self.n_inspections + 1;
        item.apply_op(&self.operation);
        item.apply_op(&Operation::Divide(3));

        if item.test_op(&self.test) {
            Some((self.truthy_target, item))
        } else {
            Some((self.falsey_target, item))
        }
    }

    fn throw(&mut self, item: Item) {
        self.items.push(item);
    }
}

#[derive(Debug)]
struct Simulation {
    round: i32,
    monkeys: Vec<Monkey>,
}

impl Simulation {
    fn do_round(&mut self) {
        for i in 0..self.monkeys.len() {
            while let Some((target, item)) = self.do_turn_on(i) {
                self.monkeys.get_mut(target as usize).unwrap().throw(item);
            }
        }

        self.round = self.round + 1;
    }

    fn do_turn_on(&mut self, monkey: usize) -> Option<(usize, Item)> {
        self.monkeys.get_mut(monkey).unwrap().do_turn()
    }
}

fn main() {
    let input = include_str!("input.txt").replace("\r", "");

    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|line| line.parse::<Monkey>().unwrap())
        .collect();

    let mut simulation = Simulation { round: 0, monkeys };

    for i in 0..20 {
        simulation.do_round();
    }

    let mut n_inspections: Vec<usize> = simulation
        .monkeys
        .iter()
        .map(|monkey| monkey.n_inspections)
        .collect();

    n_inspections.sort_by(|a, b| b.cmp(a));

    let monkey_business = n_inspections.get(0).unwrap() * n_inspections.get(1).unwrap();

    dbg!(monkey_business);
}
