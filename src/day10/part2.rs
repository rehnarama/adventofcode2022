use std::{fmt::Display, str::FromStr, time::Duration};

#[derive(Debug, Clone)]
enum Operation {
    Noop,
    Addx(i32),
}

impl Operation {
    fn cycles(&self) -> usize {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

#[derive(Debug, Clone, Default)]
struct Cpu {
    register_x: i32,
    cycle: usize,
    operation_cycle: usize,
    operation: usize,
    operations: Vec<Operation>,
}

struct CRT {
    pixels: Vec<bool>,
}

impl Cpu {
    fn start_cycle(&mut self) {
        self.cycle += 1;
        self.operation_cycle += 1;
    }

    fn end_cycle(&mut self) {
        let op = self.operations.get(self.operation).unwrap();

        if self.operation_cycle == op.cycles() {
            if let Operation::Addx(arg) = op {
                self.register_x += arg;
            }

            self.operation += 1;
            self.operation_cycle = 0;
        }
    }

    fn is_done(&self) -> bool {
        self.operation >= self.operations.len()
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("noop") {
            Ok(Operation::Noop)
        } else {
            let (_, value_str) = s.split_once(" ").ok_or(format!("Couldn't parse {}", s))?;
            let value = value_str.parse::<i32>().map_err(|e| e.to_string())?;
            Ok(Operation::Addx(value))
        }
    }
}

impl Display for CRT {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..(40 * 6) {
            let pixel = self.pixels.get(i).unwrap();

            if i != 0 && i % 40 == 0 {
                write!(f, "\n")?;
            }

            if *pixel {
                write!(f, "#")?;
            } else {
                write!(f, ".")?;
            }
        }

        Ok(())
    }
}

fn main() -> Result<(), String> {
    let input = include_str!("input.txt");

    let operations = input
        .lines()
        .map(|op| op.parse::<Operation>().unwrap())
        .collect::<Vec<Operation>>();

    let mut cpu = Cpu {
        register_x: 1,
        operations,
        ..Default::default()
    };
    let mut display = CRT {
        pixels: vec![false; 40 * 6],
    };
    print!("{}[2J", 27 as char);
    while !cpu.is_done() {
        print!("{esc}[1;1H", esc = 27 as char);

        let x = cpu.cycle % 40;
        // let y = (cpu.cycle as f32 / 40f32) as u32;

        if (x as i32).abs_diff(cpu.register_x) <= 1 {
            *display.pixels.get_mut(cpu.cycle).unwrap() = true;
        }

        println!("{}", &display);
        println!("");

        cpu.start_cycle();

        // Animate screen with some pizzaz!
        std::thread::sleep(Duration::from_millis(4));

        cpu.end_cycle();
    }

    Ok(())
}
