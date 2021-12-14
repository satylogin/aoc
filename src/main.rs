#![feature(test)]

use std::collections::HashMap;

fn freq_map() -> HashMap<(u8, u8), usize> {
    (0..26)
        .flat_map(|a| (0..26).map(move |b| ((a + 'A' as u8, b + 'A' as u8), 0)))
        .collect()
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = data.split('\n').filter(|l| !l.is_empty());

    let poly = lines.next().unwrap().bytes().collect::<Vec<_>>();
    let conversions = lines
        .map(|l| {
            let (p, s) = l.split_once(" -> ").unwrap();
            let left = p.bytes().collect::<Vec<_>>();
            ((left[0], left[1]), s.bytes().next().unwrap())
        })
        .collect::<HashMap<_, _>>();
    let mut freq = freq_map();
    poly.windows(2)
        .for_each(|w| *freq.get_mut(&(w[0], w[1])).unwrap() += 1);

    let mut new_freq = freq_map();
    for _ in 0..40 {
        new_freq.iter_mut().for_each(|(_, v)| *v = 0);
        for (k, v) in freq.iter() {
            if let Some(&c) = conversions.get(k) {
                *new_freq.get_mut(&(k.0, c)).unwrap() += v;
                *new_freq.get_mut(&(c, k.1)).unwrap() += v;
            } else {
                *new_freq.get_mut(k).unwrap() += v;
            }
        }
        std::mem::swap(&mut freq, &mut new_freq);
    }

    let mut f = HashMap::new();
    for (k, v) in freq {
        *f.entry(k.0).or_insert(0) += v;
        *f.entry(k.1).or_insert(0) += v;
    }
    let mut vals = f
        .into_iter()
        .map(|(_, v)| if v & 1 == 1 { v / 2 + 1 } else { v / 2 })
        .filter(|v| *v != 0)
        .collect::<Vec<_>>();
    vals.sort_unstable();
    println!("{}", vals[vals.len() - 1] - vals[0]);
}

extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_run(b: &mut Bencher) {
        b.iter(|| main());
    }
}
