use std::{
    ops::{Add, Sub},
    str::FromStr, collections::HashSet,
};

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl Add for Coordinate {
    type Output = Coordinate;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Sub for Coordinate {
    type Output = Coordinate;

    fn sub(self, rhs: Self) -> Self::Output {
        Coordinate {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Hash, Clone, Copy, Debug)]
struct Rope {
    head: Coordinate,
    tail: Coordinate,
}

impl Rope {
    fn move_head(&mut self, delta: Coordinate) {
        self.head = self.head + delta;
        self.update_tail();
    }

    fn update_tail(&mut self) {
        let delta = self.head - self.tail;

        if delta.y.abs() > 1 || delta.x.abs() > 1 {
            if delta.y.abs() >= 1 && delta.x.abs() >= 1 {
                self.tail.y += delta.y.signum();
                self.tail.x += delta.x.signum();
            } else if delta.y.abs() > 1 {
                self.tail.y += delta.y.signum();
            } else if delta.x.abs() > 1 {
                self.tail.x += delta.x.signum();
            }
        }
    }
}

#[derive(Debug)]
struct Move {
    delta: Coordinate,
}

fn get_move(s: &str) -> Result<Vec<Move>, &str> {
    let (direction, amount_str) = s.split_once(' ').ok_or("Couldn't split by whitespace")?;

    let amount = amount_str
        .parse::<i32>()
        .map_err(|_| "Couldn't parse amount")?;

    let r = 1..=amount;

    match direction {
        "L" => Ok(r
            .map(|_| Move {
                delta: Coordinate { x: -1, y: 0 },
            })
            .collect()),
        "R" => Ok(r
            .map(|_| Move {
                delta: Coordinate { x: 1, y: 0 },
            })
            .collect()),
        "U" => Ok(r
            .map(|_| Move {
                delta: Coordinate { x: 0, y: 1 },
            })
            .collect()),
        "D" => Ok(r
            .map(|_| Move {
                delta: Coordinate { x: 0, y: -1 },
            })
            .collect()),
        _ => Err("Unknown direction"),
    }
}

fn main() {
    let input = include_str!("input.txt");

    let mut rope = Rope {
        head: Coordinate { x: 0, y: 0 },
        tail: Coordinate { x: 0, y: 0 },
    };

    let moves = input
        .lines()
        .flat_map(|line| get_move(line).unwrap())
        .collect::<Vec<Move>>();

    let mut unique_coordinates: HashSet<Coordinate> = HashSet::new();
    unique_coordinates.insert(rope.tail);

    moves.iter().for_each(|m| {
        rope.move_head(m.delta);
        unique_coordinates.insert(rope.tail);
    });

    dbg!(unique_coordinates.len());
}
