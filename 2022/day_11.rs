struct Monkey<'expression> {
    items: Vec<i64>,
    expression: &'expression str,
    test: Test,
}

struct Test {
    div_by: i64,
    if_true: usize,
    if_false: usize,
}

fn operation(old: i64, expression: &str) -> i64 {
    let expression = expression.split(' ').collect::<Vec<_>>();
    let op1 = match expression[0] {
        "old" => old,
        num => num.parse::<i64>().unwrap(),
    };
    let op2 = match expression[2] {
        "old" => old,
        num => num.parse::<i64>().unwrap(),
    };
    match expression[1] {
        "+" => op1 + op2,
        "*" => op1 * op2,
        _ => unreachable!(),
    }
}

fn count_inspection(monkeys: &mut Vec<Monkey>, modulo: i64) -> usize {
    let mut count = vec![0; monkeys.len()];
    for _ in 0..10000 {
        for m in 0..monkeys.len() {
            count[m] += monkeys[m].items.len();
            let items = monkeys[m].items.clone();
            for old in items {
                let new = operation(old, monkeys[m].expression) % modulo;
                let to_monkey = if new % monkeys[m].test.div_by == 0 {
                    monkeys[m].test.if_true
                } else {
                    monkeys[m].test.if_false
                };
                monkeys[to_monkey].items.push(new);
            }
            monkeys[m].items.clear();
        }
    }
    count.sort_by(|x, y| y.cmp(&x));
    return count[0] * count[1];
}

fn main() {
    let last_num = |l: &str| l.split(' ').last().unwrap().parse::<i64>().unwrap();
    for input in ["test.txt", "input.txt"] {
        let mut monkeys = vec![];
        let lines = std::fs::read_to_string(input).unwrap();
        let lines = lines.split('\n').collect::<Vec<_>>();
        let modulo = (3..lines.len())
            .step_by(7)
            .map(|l| last_num(lines[l]))
            .product();
        for i in (0..lines.len()).step_by(7) {
            monkeys.push(Monkey {
                items: lines[i + 1]
                    .split_once(':')
                    .unwrap()
                    .1
                    .split(',')
                    .map(|num| num.trim().parse::<i64>().unwrap() % modulo)
                    .collect::<Vec<_>>(),
                expression: lines[i + 2].split_once('=').unwrap().1.trim(),
                test: Test {
                    div_by: last_num(lines[i + 3]),
                    if_true: last_num(lines[i + 4]) as usize,
                    if_false: last_num(lines[i + 5]) as usize,
                },
            });
        }
        println!("{}", count_inspection(&mut monkeys, modulo));
    }
}
