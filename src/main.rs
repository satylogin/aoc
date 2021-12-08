use std::collections::HashMap;

fn signal_to_digit(rewired: Vec<char>) -> HashMap<String, usize> {
    vec![
        "abefgd", "be", "abcgf", "abcef", "dcbe", "adcef", "adgfec", "abe", "abcdefg", "abcdef",
    ]
    .into_iter()
    .map(|s| {
        let s = s
            .chars()
            .map(|c| rewired[(c as u8 - 'a' as u8) as usize])
            .collect::<String>();
        sort(&s)
    })
    .enumerate()
    .map(|(n, s)| (s.to_string(), n))
    .collect()
}

fn sort(s: &str) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort();
    s.into_iter().collect()
}

fn missing(req: &Vec<char>, has: &Vec<char>) -> Vec<char> {
    req.iter()
        .filter(|c| !has.contains(c))
        .map(|c| *c)
        .collect()
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
    signal[1] = missing(&(nos[0].chars().collect()), &vec![signal[4]])[0];
    signal[0] = missing(&(nos[1].chars().collect()), &vec![signal[1], signal[4]])[0];
    signal[2] = missing(
        &(nos[2].chars().collect()),
        &vec![signal[1], signal[3], signal[4]],
    )[0];
    signal[5] = signal[0];
    signal[5] = missing(&(nos[9].chars().collect()), &(signal.clone()))[0];
    signal
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines: Vec<&str> = data.split('\n').filter(|l| l.len() > 0).collect();
    let s = lines
        .into_iter()
        .map(|line| {
            let mut splitter = line.split('|');
            let numbers = splitter.next().unwrap().split(' ').filter(|c| c.len() > 0);
            let mapping = signal_to_digit(generate_signal(&mut numbers.collect::<Vec<_>>()));
            let output = splitter.next().unwrap().split(' ').filter(|o| o.len() > 0);
            output.fold(0, |num, n| num * 10 + mapping[&sort(n)])
        })
        .sum::<usize>();
    println!("{}", s);
}
