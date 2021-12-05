fn main() {
    let bins: Vec<Vec<char>> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|s| s.chars().collect())
        .collect();
    let bins: Vec<usize> = (0..bins[0].len())
        .map(|i| {
            let mut f = [0, 0];
            for b in &bins {
                f[(b[i] as u8 - '0' as u8) as usize] += 1;
            }
            if f[0] > f[1] {
                0
            } else if f[1] > f[0] {
                1
            } else {
                panic!("no not this");
            }
        })
        .collect();
    let nbins: Vec<usize> = bins.iter().map(|c| 1 - c).collect();

    let a: String = bins
        .into_iter()
        .map(|i| (i as u8 + '0' as u8) as char)
        .collect();
    let b: String = nbins
        .into_iter()
        .map(|i| (i as u8 + '0' as u8) as char)
        .collect();
    let x = usize::from_str_radix(&a, 2).unwrap();
    let y = usize::from_str_radix(&b, 2).unwrap();
    dbg!(x * y);
}
