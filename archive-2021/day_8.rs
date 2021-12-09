use std::collections::HashMap;

fn signal_to_digit(rewired: Vec<char>) -> HashMap<String, usize> {
    let c_rewired = |c: char| rewired[(c as u8 - 'a' as u8) as usize];
    vec![
        "abefgd", "be", "abcgf", "abcef", "dcbe", "adcef", "adgfec", "abe", "abcdefg", "abcdef",
    ]
    .into_iter()
    .map(|s| sort(&s.chars().map(c_rewired).collect::<String>()))
    .zip(0..10)
    .collect()
}

fn sort(s: &str) -> String {
    let mut s = s.chars().collect::<Vec<_>>();
    s.sort_unstable();
    s.into_iter().collect()
}

fn missing(req: Vec<char>, has: Vec<char>) -> Vec<char> {
    req.into_iter().filter(|c| !has.contains(&c)).collect()
}

fn generate_wiring(signals: &mut Vec<&str>) -> Vec<char> {
    let mut w = vec!['0'; 7];

    let mut freq: HashMap<char, usize> = "abcdefg".chars().map(|c| (c, 0)).collect();
    signals.into_iter().for_each(|n| {
        n.chars().for_each(|c| {
            *freq.get_mut(&c).unwrap() += 1;
        });
    });
    freq.iter().for_each(|(c, f)| match *f {
        9 => w[4] = *c,
        6 => w[3] = *c,
        4 => w[6] = *c,
        _ => {}
    });

    signals.sort_unstable_by(|a, b| a.len().cmp(&b.len()));
    w[1] = missing(signals[0].chars().collect(), vec![w[4]])[0];
    w[0] = missing(signals[1].chars().collect(), vec![w[1], w[4]])[0];
    w[2] = missing(signals[2].chars().collect(), vec![w[1], w[3], w[4]])[0];
    w[5] = missing(signals[9].chars().collect(), w.clone())[0];

    w
}

fn main() {
    let now = std::time::Instant::now();
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split('\n').filter(|l| l.len() > 0);
    let resolved = lines.map(|line| {
        let mut splitter = line.split('|');
        let numbers = splitter.next().unwrap().split(' ').filter(|c| c.len() > 0);
        let output = splitter.next().unwrap().split(' ').filter(|o| o.len() > 0);
        let std = signal_to_digit(generate_wiring(&mut numbers.collect::<Vec<_>>()));
        output.fold(0, |num, n| num * 10 + std[&sort(n)])
    });
    println!("ans: {}", resolved.sum::<usize>());
    println!("elapsed: {}", now.elapsed().as_millis());
}
