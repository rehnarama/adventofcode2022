use colored::Colorize;
use std::{
    fmt::{self, Write},
    ops::{Add, Sub},
    str::FromStr,
};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
struct Point {
    x: usize,
    y: usize,
}
impl Point {
    fn from_string(str: String) -> Result<Point, String> {
        let (x_str, y_str) = match str.trim().split_once(",") {
            Some(s) => s,
            None => return Err(format!("Couldn't parse to point, str='{}'", str)),
        };

        let x = match x_str.parse::<usize>() {
            Ok(x) => x,
            Err(e) => return Err(format!("{}", e)),
        };
        let y = match y_str.parse::<usize>() {
            Ok(y) => y,
            Err(e) => return Err(format!("{}", e)),
        };

        Ok(Point { x, y })
    }

    fn distance(&self, other: &Point) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}
impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone)]
struct Grid {
    data: Vec<Vec<usize>>,
}
impl Grid {
    fn new(width: usize, height: usize) -> Grid {
        Grid {
            data: vec![vec![0; height]; width],
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<&usize> {
        match self.data.get(x) {
            Some(column) => column.get(y),
            None => None,
        }
    }

    fn set(&mut self, x: usize, y: usize, number: usize) -> Result<(), String> {
        match self.get_mut(x, y) {
            Some(cell) => {
                *cell = number;
                Ok(())
            }
            None => Err(format!("Couldn't find position ({}, {})", x, y)),
        }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut usize> {
        match self.data.get_mut(x) {
            Some(column) => column.get_mut(y),
            None => None,
        }
    }

    fn width(&self) -> usize {
        self.data.len()
    }
    fn height(&self) -> usize {
        self.data.get(0).unwrap().len()
    }
}

impl fmt::Debug for Grid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{\n")?;
        for y in 0..self.height() {
            f.write_char('\t')?;
            for x in 0..self.width() {
                f.write_str(&self.get(x, y).unwrap().to_string())?;
            }
            f.write_char('\n')?;
        }
        f.write_str("}")?;

        Ok(())
    }
}

struct Trees {
    grid: Grid,
}

impl FromStr for Trees {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data: Vec<(usize, usize, usize)> = s
            .lines()
            .enumerate()
            .flat_map(|(y, line)| {
                line.chars()
                    .map(|char| char.to_digit(10).unwrap() as usize)
                    .enumerate()
                    .map(move |(x, height)| (x, y, height))
            })
            .collect();

        let (x_upper_bound, y_upper_bound): (usize, usize) = data
            .iter()
            .copied()
            .fold((0, 0), |(biggest_x, biggest_y), (x, y, _)| {
                (biggest_x.max(x), biggest_y.max(y))
            });

        let mut grid = Grid::new(x_upper_bound + 1, y_upper_bound + 1);
        data.iter().copied().for_each(|(x, y, height)| {
            grid.set(x, y, height).unwrap();
        });

        Ok(Trees { grid })
    }
}

impl Trees {
    fn scenic_score(&self, point: Point) -> usize {
        self.distance_top(point)
            * self.distance_down(point)
            * self.distance_left(point)
            * self.distance_right(point)
    }

    fn distance_top(&self, point: Point) -> usize {
        let tree = self.grid.get(point.x, point.y).unwrap();

        let stop_point = (0..point.y).rev().map(|y| Point { x: point.x, y }).fold(
            point - Point { x: 0, y: 1 },
            |point, next_point| {
                let point_height = self.grid.get(point.x, point.y).unwrap();

                if point_height >= tree {
                    point
                } else {
                    next_point
                }
            },
        );

        point.distance(&stop_point)
    }

    fn distance_down(&self, point: Point) -> usize {
        let tree = self.grid.get(point.x, point.y).unwrap();

        let stop_point = (point.y..self.grid.height())
            .skip(2)
            .map(|y| Point { x: point.x, y })
            .fold(point + Point { x: 0, y: 1 }, |point, next_point| {
                let point_height = self.grid.get(point.x, point.y).unwrap();

                if point_height >= tree {
                    point
                } else {
                    next_point
                }
            });

        point.distance(&stop_point)
    }

    fn distance_left(&self, point: Point) -> usize {
        let tree = self.grid.get(point.x, point.y).unwrap();

        let stop_point = (0..point.x).rev().map(|x| Point { x, y: point.y }).fold(
            point - Point { x: 1, y: 0 },
            |point, next_point| {
                let point_height = self.grid.get(point.x, point.y).unwrap();

                if point_height >= tree {
                    point
                } else {
                    next_point
                }
            },
        );

        point.distance(&stop_point)
    }

    fn distance_right(&self, point: Point) -> usize {
        let tree = self.grid.get(point.x, point.y).unwrap();

        let stop_point = (point.x..self.grid.width())
            .skip(2)
            .map(|x| Point { x, y: point.y })
            .fold(point + Point { x: 1, y: 0 }, |point, next_point| {
                let point_height = self.grid.get(point.x, point.y).unwrap();

                if point_height >= tree {
                    point
                } else {
                    next_point
                }
            });

        point.distance(&stop_point)
    }
}

impl fmt::Debug for Trees {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("{\n")?;
        for y in 0..self.grid.height() {
            f.write_char('\t')?;
            for x in 0..self.grid.width() {
                let point = Point { x, y };
                f.write_str(&self.grid.get(x, y).unwrap().to_string())?;
            }
            f.write_char('\n')?;
        }
        f.write_str("}")?;

        Ok(())
    }
}

fn main() {
    let input = include_str!("input.txt");

    let trees = input.parse::<Trees>().unwrap();

    let score = (1..(trees.grid.width() - 1))
        .flat_map(|x| (1..(trees.grid.height() - 1)).map(move |y| (x, y)))
        .map(|(x, y)| trees.scenic_score(Point { x, y }))
        .fold(0, |current, score| current.max(score));
        // .collect::<Vec<usize>>();
    dbg!(score);

    // dbg!(visible);

    // dbg!(trees.distance_top(Point { x: 2, y: 1 })); 
    // dbg!(trees.distance_down(Point { x: 2, y: 1 }));
    // dbg!(trees.distance_left(Point { x: 2, y: 1 }));
    // dbg!(trees.distance_right(Point { x: 2, y: 1 }));
}
