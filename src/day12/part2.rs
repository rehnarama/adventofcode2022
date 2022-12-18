use std::{collections::HashMap, fmt::Display};

struct Map {
    heights: HashMap<(usize, usize), char>,
    size: (usize, usize),
    start: Vec<(usize, usize)>,
    end: (usize, usize),
    scores: HashMap<(usize, usize), u32>,
}

impl Map {
    fn new(heights: HashMap<(usize, usize), char>) -> Map {
        let width = heights.keys().fold(0, |acc, element| acc.max(element.0));
        let height = heights.keys().fold(0, |acc, element| acc.max(element.1));
        let start = heights
            .iter()
            .filter(|(_, &height)| height == 'S' || height == 'a')
            .map(|(&coord, _)| coord)
            .collect();
        let end = *heights
            .iter()
            .find(|(_, &height)| height == 'E')
            .map(|(coord, _)| coord)
            .unwrap();

        let mut scores = HashMap::new();
        scores.insert(end, 0);
        Map {
            heights: heights
                .iter()
                .map(|(&coord, &height)| {
                    let height = match height {
                        'S' => 'a',
                        'E' => 'z',
                        _ => height,
                    };
                    (coord, height)
                })
                .collect(),
            size: (width, height),
            start,
            end,
            scores,
        }
    }

    fn calculate_scores(&mut self, to: (usize, usize)) {
        let to_score = *self.scores.get(&to).unwrap();

        let surrounding = self.get_surrounding(to);

        for from in surrounding {
            let from_score = to_score + 1;

            let current_from_score = self.scores.get(&from);

            if self.is_eligible_move(&from, &to) {
                if current_from_score.is_some() && *current_from_score.unwrap() <= from_score {
                    continue;
                }

                self.scores.insert(from, from_score);
                self.calculate_scores(from);
            }
        }
    }

    fn is_eligible_move(&self, from: &(usize, usize), to: &(usize, usize)) -> bool {
        let from = *self.heights.get(from).unwrap();
        let to = *self.heights.get(to).unwrap();

        (to as i32) - (from as i32) <= 1
    }

    fn get_surrounding(&self, pos: (usize, usize)) -> Vec<(usize, usize)> {
        let mut surrounding = Vec::new();
        for x in vec![-1, 0, 1] {
            for y in vec![-1, 0, 1] {
                if !(x == 0 || y == 0) || (x == 0 && y == 0) {
                    continue;
                }

                let target_x = pos.0 as i32 + x;
                let target_y = pos.1 as i32 + y;

                let in_range_x = target_x >= 0 && target_x <= self.size.0 as i32;
                let in_range_y = target_y >= 0 && target_y <= self.size.1 as i32;
                if in_range_x && in_range_y {
                    surrounding.push((target_x.try_into().unwrap(), target_y.try_into().unwrap()));
                }
            }
        }

        surrounding
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..=self.size.1 {
            for x in 0..=self.size.0 {
                write!(
                    f,
                    ".{:0>4}",
                    self.scores
                        .get(&(x, y))
                        .map(|s| s.to_string())
                        .unwrap_or("-".to_string())
                )?;
            }
            write!(f, "\n")?;
        }

        Ok(())
    }
}

fn main() {
    let input = include_str!("input.txt");

    let heights: HashMap<(usize, usize), char> = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .map(move |(x, height)| ((x, y), height))
        })
        .collect();
    let mut map = Map::new(heights);
    map.calculate_scores(map.end);

    let n_steps = map
        .start
        .iter()
        .map(|start| map.scores.get(start))
        .filter(|op| op.is_some())
        .fold(u32::MAX, |min_steps, steps| min_steps.min(*steps.unwrap()));

    dbg!(n_steps);
}
