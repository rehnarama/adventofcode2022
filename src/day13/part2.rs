use std::{cmp::Ordering, fmt::Display, str::FromStr};

#[derive(Debug, Clone)]
enum Input {
    Value(u32),
    List(Vec<Input>),
}

impl FromStr for Input {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bracket_count = 0;

        let mut data: Vec<Input> = Vec::new();

        let mut start_bracket_positions: Vec<usize> = Vec::new();
        let mut start_number_position: Option<usize> = None;

        for (i, c) in s.chars().enumerate() {
            if c == '[' {
                bracket_count = bracket_count + 1;
                if bracket_count == 2 {
                    start_bracket_positions.push(i);
                }
            }
            if c == ']' {
                if bracket_count == 2 {
                    let start = start_bracket_positions.pop().unwrap();
                    let end = i;

                    let slice = &s[start..=end];
                    let list = slice.parse::<Input>().unwrap();
                    data.push(list);
                }

                bracket_count = bracket_count - 1;
            } else if bracket_count == 1 && start_number_position.is_none() && c >= '0' && c <= '9'
            {
                start_number_position = Some(i);
            }
            if start_number_position.is_some() && (c == ',' || c == ']') {
                let start = start_number_position.unwrap();
                let end = i;

                let slice = &s[start..end];
                let value = slice.parse::<u32>().unwrap();
                data.push(Input::Value(value));
                start_number_position = None;
            }
        }

        Ok(Input::List(data))
    }
}

impl Display for Input {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Input::Value(number) => write!(f, "{}", number),
            Input::List(inputs) => {
                write!(f, "[")?;
                for (i, input) in inputs.iter().enumerate() {
                    write!(f, "{}", &input)?;
                    if i != inputs.len() - 1 {
                        write!(f, ",")?;
                    }
                }
                write!(f, "]")?;

                Ok(())
            }
        }
    }
}

impl Input {
    fn is_in_order(&self, other: &Input) -> Option<bool> {
        match (self, other) {
            (Input::Value(l), Input::Value(r)) => {
                if l < r {
                    Some(true)
                } else if l > r {
                    Some(false)
                } else {
                    None
                }
            }
            (Input::List(l), Input::List(r)) => {
                let piecewise_result =
                    l.iter()
                        .zip(r.iter())
                        .fold(None, |acc, (l_value, r_value)| {
                            if acc.is_none() {
                                l_value.is_in_order(r_value)
                            } else {
                                acc
                            }
                        });

                if piecewise_result.is_some() {
                    piecewise_result
                } else {
                    Input::Value(l.len() as u32).is_in_order(&Input::Value(r.len() as u32))
                }
            }
            (Input::Value(_), Input::List(_)) => Input::List(vec![self.clone()]).is_in_order(other),
            (Input::List(_), Input::Value(_)) => {
                self.is_in_order(&Input::List(vec![other.clone()]))
            }
        }
    }
}

impl PartialEq for Input {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Value(l0), Self::Value(r0)) => l0 == r0,
            (Self::List(l0), Self::List(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl Eq for Input {}
impl PartialOrd for Input {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cmp = self.is_in_order(other);
        cmp.map(|result| {
            if result {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        })
    }
}
impl Ord for Input {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn main() {
    let input = include_str!("input.txt")
        .replace("\r", "")
        .replace("\n\n", "\n");

    let divider1 = "[[2]]".parse::<Input>().unwrap();
    let divider2 = "[[6]]".parse::<Input>().unwrap();

    let mut packets = input
        .lines()
        .map(|line| line.parse::<Input>().unwrap())
        .chain(vec![divider1.clone(), divider2.clone()])
        .collect::<Vec<Input>>();
    packets.sort();



    let divider_product: usize = packets.iter().enumerate()
        .filter(|(i, item)| item == &&divider1 || item == &&divider2)
        .map(|(i, _)| i + 1)
        .product();

    dbg!(divider_product);
}
