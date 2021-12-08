use std::collections::HashMap;

fn resolve(base: &Vec<char>, n: &str) -> i64 {
    let base: String = base.iter().collect();
    let mut patterns = vec![vec![0; 7]; 10];
    vec![
        vec![0, 1, 4, 5, 6, 3],
        vec![1, 4],
        vec![0, 1, 2, 6, 5],
        vec![0, 1, 2, 4, 5],
        vec![3, 2, 1, 4],
        vec![0, 3, 2, 4, 5],
        vec![0, 3, 6, 5, 4, 2],
        vec![0, 1, 4],
        vec![0, 1, 2, 3, 4, 5, 6],
        vec![0, 1, 2, 3, 4, 5],
    ]
    .into_iter()
    .enumerate()
    .for_each(|(i, n)| n.into_iter().for_each(|s| patterns[i][s] = 1));
    let mut slate = vec![0; 7];
    n.chars().for_each(|c| {
        slate[base.find(c).unwrap()] = 1;
    });
    for i in 0..patterns.len() {
        if patterns[i] == slate {
            return i as i64;
        }
    }
    -1
}

fn valid(base: &Vec<char>, nos: &Vec<&str>) -> bool {
    let mut got = vec![false; 10];
    for n in nos {
        let i = resolve(base, n);
        if i == -1 {
            return false;
        }
        got[i as usize] = true;
    }
    got.into_iter().all(|e| e)
}

fn missing(req: &Vec<char>, has: &Vec<char>) -> Vec<char> {
    let mut m = vec![];
    for c in req {
        if !has.contains(c) {
            m.push(*c);
        }
    }
    m
}

fn generate_signal(nos: &mut Vec<&str>) -> Vec<char> {
    let mut signal = vec!['a'; 7];

    let mut freq: HashMap<char, usize> = "abcdefg".chars().map(|c| (c, 0)).collect();
    nos.into_iter().for_each(|n| {
        n.chars().for_each(|c| {
            *freq.get_mut(&c).unwrap() += 1;
        });
    });
    freq.iter().for_each(|(c, f)| match *f {
        9 => signal[4] = *c,
        6 => signal[3] = *c,
        4 => signal[6] = *c,
        _ => {}
    });
    nos.sort_by(|a, b| a.len().cmp(&b.len()));
    let one: Vec<char> = nos[0].chars().collect();
    signal[1] = missing(&one, &vec![signal[4]])[0];
    let seven: Vec<char> = nos[1].chars().collect();
    signal[0] = missing(&seven, &vec![signal[1], signal[4]])[0];
    let four: Vec<char> = nos[2].chars().collect();
    signal[2] = missing(&four, &vec![signal[1], signal[3], signal[4]])[0];
    signal[5] = signal[0];
    signal[5] = missing(&(nos[9].chars().collect()), &(signal.clone()))[0];
    signal
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.split('\n').collect();
    let s = lines
        .into_iter()
        .filter(|line| line.len() > 0)
        .map(|line| {
            let mut splitter = line.split('|');
            let mut nos = splitter
                .next()
                .unwrap()
                .split(' ')
                .filter(|c| c.len() > 0)
                .collect::<Vec<_>>();
            let signal = generate_signal(&mut nos);
            assert!(valid(&signal, &nos));
            let mut num = 0;
            splitter
                .next()
                .unwrap()
                .split(' ')
                .filter(|n| n.len() > 0)
                .for_each(|n| num = num * 10 + resolve(&signal, n));
            num
        })
        .sum::<i64>();
    println!("{}", s);
}
