use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::{write, Display},
    ops::Add,
    str::FromStr,
};

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

impl Rect {
    fn encapsulate(&mut self, point: &Point) {
        if self.left > point.x {
            self.left = point.x;
        } else if self.right < point.x {
            self.right = point.x;
        }

        if self.top > point.y {
            self.top = point.y;
        } else if self.bottom < point.y {
            self.bottom = point.y;
        }
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl FromStr for Point {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(",").unwrap();

        Ok(Point {
            x: x.parse::<i32>().unwrap(),
            y: y.parse::<i32>().unwrap(),
        })
    }
}

impl From<(i32, i32)> for Point {
    fn from(p: (i32, i32)) -> Self {
        Point { x: p.0, y: p.1 }
    }
}

struct Map {
    sand_point: Point,
    rocks: HashSet<Point>,
    sand: HashSet<Point>,
    bounds: Rect,
}

impl Map {
    fn new(rocks: HashSet<Point>) -> Map {
        let mut bounds = Rect {
            left: i32::MAX,
            right: i32::MIN,
            top: i32::MAX,
            bottom: i32::MIN,
        };

        rocks.iter().for_each(|p| bounds.encapsulate(p));

        Map {
            sand_point: "500,0".parse().unwrap(),
            rocks,
            sand: HashSet::new(),
            bounds,
        }
    }

    fn occupied(&self, pos: Point) -> bool {
        self.sand.contains(&pos) || self.rocks.contains(&pos)
    }

    fn spawn_sand(&mut self) {
        let mut position = Some(self.sand_point);
        let mut old_position = position.clone();

        loop {
            position = self.simulate_step(position.unwrap());

            if position.is_none() || position == old_position {
                break;
            } else {
                old_position = position;
            }
        }

        if let Some(position) = position {
            self.sand.insert(position);
        }
    }

    fn simulate_step(&self, pos: Point) -> Option<Point> {
        let mut pos = self.find_projection_point(pos);

        if pos.is_none() {
            return None;
        }

        let mut pos = pos.unwrap();

        let down_left: Point = (-1, 1).into();
        let down_right: Point = (1, 1).into();

        if !self.occupied(pos + down_left) {
            pos = pos + down_left;
        } else if !self.occupied(pos + down_right) {
            pos = pos + down_right;
        }

        Some(pos)
    }

    fn find_projection_point(&self, pos: Point) -> Option<Point> {
        for y in pos.y..=self.bounds.bottom {
            if self.occupied((pos.x, y + 1).into()) {
                return Some((pos.x, y).into());
            }
        }

        None
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_point = self
            .rocks
            .iter()
            .chain(vec![&self.sand_point])
            .fold(Point::from((i32::MAX, i32::MAX)), |acc, element| {
                Point::from((acc.x.min(element.x), acc.y.min(element.y)))
            });
        let max_point = self
            .rocks
            .iter()
            .chain(vec![&self.sand_point])
            .fold(Point::from((i32::MIN, i32::MIN)), |acc, element| {
                Point::from((acc.x.max(element.x), acc.y.max(element.y)))
            });

        for y in min_point.y..=max_point.y {
            for x in min_point.x..=max_point.x {
                if self.sand_point == (x, y).into() {
                    write!(f, "+")?;
                } else if self.sand.contains(&(x, y).into()) {
                    write!(f, "o")?;
                } else if self.rocks.contains(&(x, y).into()) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let input = include_str!("input.txt");

    let rocks: HashSet<Point> = input
        .lines()
        .flat_map(|line| {
            let trace_points: Vec<Point> = line
                .split(" -> ")
                .map(|p| p.parse::<Point>().unwrap())
                .collect();

            trace_points
                .windows(2)
                .flat_map(|window| {
                    let p1 = &window[0];
                    let p2 = &window[1];
                    let min: Point = (p1.x.min(p2.x), p1.y.min(p2.y)).into();
                    let max: Point = (p1.x.max(p2.x), p1.y.max(p2.y)).into();
                    (min.x..=max.x)
                        .flat_map(move |x| (min.y..=max.y).map(move |y| Point::from((x, y))))
                })
                .collect::<Vec<Point>>()
        })
        .collect();

    let mut map = Map::new(rocks);

    let mut n_sand = 0;
    loop {
        map.spawn_sand();
        if n_sand != map.sand.len() {
            n_sand = map.sand.len();
        } else {
            break;
        }
    }

    dbg!(n_sand);
}
