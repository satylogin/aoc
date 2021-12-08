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

fn generate_wiring(signals: &mut Vec<&str>) -> Vec<char> {
    let mut wiring = vec!['0'; 7];

    let mut freq: HashMap<char, usize> = "abcdefg".chars().map(|c| (c, 0)).collect();
    signals.into_iter().for_each(|n| {
        n.chars().for_each(|c| {
            *freq.get_mut(&c).unwrap() += 1;
        });
    });
    freq.iter().for_each(|(c, f)| match *f {
        9 => wiring[4] = *c,
        6 => wiring[3] = *c,
        4 => wiring[6] = *c,
        _ => {}
    });

    signals.sort_by(|a, b| a.len().cmp(&b.len()));
    wiring[1] = missing(&(signals[0].chars().collect()), &vec![wiring[4]])[0];
    wiring[0] = missing(&(signals[1].chars().collect()), &vec![wiring[1], wiring[4]])[0];
    wiring[2] = missing(
        &(signals[2].chars().collect()),
        &vec![wiring[1], wiring[3], wiring[4]],
    )[0];
    wiring[5] = missing(&(signals[9].chars().collect()), &(wiring.clone()))[0];

    wiring
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split('\n').filter(|l| l.len() > 0);
    let resolved = lines.map(|line| {
        let mut splitter = line.split('|');
        let numbers = splitter.next().unwrap().split(' ').filter(|c| c.len() > 0);
        let output = splitter.next().unwrap().split(' ').filter(|o| o.len() > 0);
        let std = signal_to_digit(generate_wiring(&mut numbers.collect::<Vec<_>>()));
        output.fold(0, |num, n| num * 10 + std[&sort(n)])
    });
    println!("{}", resolved.sum::<usize>());
}
