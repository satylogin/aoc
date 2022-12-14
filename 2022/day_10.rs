use std::iter::FromIterator;

enum Operation {
    AddX,
    NoOp,
}

struct Instruction {
    operation: Operation,
    operands: Option<i64>,
}

fn main() {
    for input in ["test.txt", "input.txt"] {
        let instructions = std::fs::read_to_string(input)
            .unwrap()
            .split('\n')
            .filter(|l| !l.is_empty())
            .map(|l| {
                if l.starts_with("a") {
                    Instruction {
                        operation: Operation::AddX,
                        operands: Some(l.split_once(' ').unwrap().1.parse::<i64>().unwrap()),
                    }
                } else {
                    Instruction {
                        operation: Operation::NoOp,
                        operands: None,
                    }
                }
            })
            .collect::<Vec<_>>();
        draw(instructions.into_iter());
    }
}

fn _sum_signal_strengths(instructions: &Vec<Instruction>) -> i64 {
    let mut times = std::collections::BTreeSet::from_iter(vec![20, 60, 100, 140, 180, 220]);
    let mut x = 1;
    let mut sum = 0;
    let mut time = 0;
    for instruction in instructions {
        time += match instruction.operation {
            Operation::AddX => 2,
            Operation::NoOp => 1,
        };
        if let Some(&interesting_time) = times.iter().next() {
            if interesting_time <= time {
                sum += interesting_time * x;
                times.remove(&interesting_time);
            }
        }
        if let Some(val) = instruction.operands {
            x += val;
        }
    }
    for interesting_time in times {
        sum += interesting_time * x;
    }
    sum
}

fn draw(mut instructions: std::vec::IntoIter<Instruction>) {
    let mut screen = vec![vec!['.'; 40]; 6];
    let mut sprite = 1;
    let mut time_for_next_instruction = 1;
    let mut time_for_apply_add = 0;
    let mut to_add = 0;
    for time in 1..241 {
        if time == time_for_apply_add {
            sprite += to_add;
        }
        let pixel = (time - 1) % 40;
        if (sprite as i32 - pixel as i32).abs() <= 1 {
            screen[(time - 1) / 40][pixel] = '#';
        }
        if time == time_for_next_instruction {
            if let Some(instruction) = instructions.next() {
                time_for_next_instruction += match instruction.operation {
                    Operation::NoOp => 1,
                    Operation::AddX => {
                        to_add = instruction.operands.unwrap();
                        time_for_apply_add = time + 2;
                        2
                    }
                };
            }
        }
    }
    display_screen(&screen);
}

fn display_screen(screen: &Vec<Vec<char>>) {
    for r in screen {
        println!("{}", r.iter().collect::<String>());
    }
    println!();
}
