use std::{
    collections::HashSet,
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Hash, Clone, Copy, Debug, Eq, PartialEq, Default)]
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

#[derive(Hash, Clone, Debug)]
struct Rope {
    knots: Vec<Coordinate>,
}

impl Rope {
    fn move_head(&mut self, delta: Coordinate) {
        let head = self.knots.get_mut(0).unwrap();
        *head = *head + delta;

        self.update_tail();
    }

    fn update_tail(&mut self) {
        for i in 0..(self.knots.len() - 1) {
            let head = self.knots.get(i).unwrap().clone();
            let tail = self.knots.get_mut(i + 1).unwrap();

            let delta = head - *tail;

            if delta.y.abs() > 1 || delta.x.abs() > 1 {
                if delta.y.abs() >= 1 && delta.x.abs() >= 1 {
                    tail.y += delta.y.signum();
                    tail.x += delta.x.signum();
                } else if delta.y.abs() > 1 {
                    tail.y += delta.y.signum();
                } else if delta.x.abs() > 1 {
                    tail.x += delta.x.signum();
                }
            }
        }
    }

    fn get_tail(&self) -> &Coordinate {
        self.knots.last().unwrap()
    }

    fn new(n_knots: i32) -> Rope {
        Rope {
            knots: vec![Coordinate::default(); n_knots.try_into().unwrap()],
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

    let mut rope = Rope::new(10);

    let moves = input
        .lines()
        .flat_map(|line| get_move(line).unwrap())
        .collect::<Vec<Move>>();

    let mut unique_coordinates: HashSet<Coordinate> = HashSet::new();
    unique_coordinates.insert(rope.get_tail().clone());

    moves.iter().for_each(|m| {
        rope.move_head(m.delta);
        unique_coordinates.insert(rope.get_tail().clone());
    });

    dbg!(unique_coordinates.len());
}
