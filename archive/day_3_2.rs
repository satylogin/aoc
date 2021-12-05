fn oxygen(mut bins: Vec<Vec<char>>) -> usize {
    for i in 0..bins[0].len() {
        let mut f = [0, 0];
        for b in &bins {
            f[(b[i] as u8 - '0' as u8) as usize] += 1;
        }
        let keep = if f[1] >= f[0] { '1' } else { '0' };
        bins = bins.into_iter().filter(|b| b[i] == keep).collect();
        if bins.len() == 1 {
            break;
        }
    }
    assert!(bins.len() == 1);
    let s: String = bins[0].iter().map(|c| *c).collect();
    usize::from_str_radix(&s, 2).unwrap()
}

fn carbon(mut bins: Vec<Vec<char>>) -> usize {
    for i in 0..bins[0].len() {
        let mut f = [0, 0];
        for b in &bins {
            f[(b[i] as u8 - '0' as u8) as usize] += 1;
        }
        let keep = if f[0] <= f[1] { '0' } else { '1' };
        bins = bins.into_iter().filter(|b| b[i] == keep).collect();
        if bins.len() == 1 {
            break;
        }
    }
    assert!(bins.len() == 1);
    let s: String = bins[0].iter().map(|c| *c).collect();
    usize::from_str_radix(&s, 2).unwrap()
}
fn main() {
    let bins: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();

    dbg!(oxygen(bins.clone()) * carbon(bins.clone()));
}
