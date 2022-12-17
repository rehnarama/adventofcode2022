use std::str::FromStr;

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

    let mut signal_strength_sum: i32 = 0;

    while !cpu.is_done() {
        cpu.start_cycle();

        if cpu.cycle == 20
            || cpu.cycle == 60
            || cpu.cycle == 100
            || cpu.cycle == 140
            || cpu.cycle == 180
            || cpu.cycle == 220
        {
            signal_strength_sum += cpu.cycle as i32 * cpu.register_x;
        }
        cpu.end_cycle();
    }

    dbg!(signal_strength_sum);

    // .map(|op| {
    //     cpu.execute(op);
    //     dbg!(&cpu);
    //     cpu.clone()
    // })
    // .filter(|result| {
    //     result.cycle == 20
    //         || result.cycle == 60
    //         || result.cycle == 100
    //         || result.cycle == 140
    //         || result.cycle == 180
    //         || result.cycle == 220
    // })
    // .collect::<Vec<Cpu>>();

    Ok(())
}
