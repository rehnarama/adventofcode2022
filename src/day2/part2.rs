#[derive(Eq, PartialEq, Clone, Copy)]
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

impl Move {
    fn calculate_move(other_move: Move, desired_result: RoundResult) -> Move {
        match desired_result {
            RoundResult::Draw => other_move.clone(),
            RoundResult::Win => other_move.winning_move(),
            RoundResult::Loose => other_move.loosing_move(),
        }
    }

    fn winning_move(&self) -> Move {
        match self {
            Move::Paper => Move::Scissor,
            Move::Scissor => Move::Rock,
            Move::Rock => Move::Paper,
        }
    }

    fn loosing_move(&self) -> Move {
        match self {
            Move::Paper => Move::Rock,
            Move::Scissor => Move::Paper,
            Move::Rock => Move::Scissor,
        }
    }
}

impl TryFrom<&str> for Move {
    type Error = String;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissor),
            _ => Err(format!("Couldn't convert {} to a Move", c)),
        }
    }
}

#[derive(Eq, PartialEq)]
enum RoundResult {
    Win,
    Loose,
    Draw,
}

impl TryFrom<&str> for RoundResult {
    type Error = String;

    fn try_from(c: &str) -> Result<Self, Self::Error> {
        match c {
            "X" => Ok(RoundResult::Loose),
            "Y" => Ok(RoundResult::Draw),
            "Z" => Ok(RoundResult::Win),
            _ => Err(format!("Couldn't convert {} to a RoundResult", c)),
        }
    }
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

fn main() {
    let input = include_str!("./input.txt");

    let score: usize = input
        .lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(other_move, desired_result)| {
            (
                Move::try_from(other_move).unwrap(),
                RoundResult::try_from(desired_result).unwrap(),
            )
        })
        .map(|(other_move, desired_result)| Round {
            other_move,
            my_move: Move::calculate_move(other_move, desired_result),
        })
        .map(|round| round.score())
        .sum();

    dbg!(score);
}
