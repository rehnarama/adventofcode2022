#[derive(Eq, PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissor,
}

impl Move {
    fn score(&self) -> usize {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissor => 3,
        }
    }
}

#[derive(Eq, PartialEq)]
enum RoundResult {
    Win,
    Loose,
    Draw,
}

impl RoundResult {
    fn score(&self) -> usize {
        match self {
            RoundResult::Loose => 0,
            RoundResult::Draw => 3,
            RoundResult::Win => 6,
        }
    }
}

struct Round {
    other_move: Move,
    my_move: Move,
}

impl Round {
    fn result(&self) -> RoundResult {
        if self.my_move == self.other_move {
            return RoundResult::Draw;
        } else {
            if self.other_move == Move::Rock && self.my_move == Move::Paper
                || self.other_move == Move::Paper && self.my_move == Move::Scissor
                || self.other_move == Move::Scissor && self.my_move == Move::Rock
            {
                RoundResult::Win
            } else {
                RoundResult::Loose
            }
        }
    }

    fn score(&self) -> usize {
        let move_score = self.my_move.score();
        let match_score = self.result().score();

        move_score + match_score
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissor),
            _ => Err(format!("Couldn't convert {} to a Move", c)),
        }
    }
}

fn main() {
    let input = include_str!("./input.txt");

    let score: usize = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(other_move, my_move)| {
            (
                Move::try_from(other_move).unwrap(),
                Move::try_from(my_move).unwrap(),
            )
        })
        .map(|(other_move, my_move)| Round {
            other_move,
            my_move,
        })
        .map(|round| round.score())
        .sum();

    dbg!(score);
}
