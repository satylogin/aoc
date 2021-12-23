use std::cmp::{max, min};
use std::collections::HashMap;

fn extract(s: &str) -> [i64; 2] {
    let r = s.split_once('=').unwrap().1.split_once("..").unwrap();
    [r.0.parse().unwrap(), r.1.parse().unwrap()]
}

fn intersection(mut a: [[i64; 2]; 3], mut b: [[i64; 2]; 3]) -> Option<[[i64; 2]; 3]> {
    for i in 0..3 {
        let (l, r) = (max(a[i][0], b[i][0]), min(a[i][1], b[i][1]));
        if l > r {
            return None;
        }
        a[i] = [l, r];
        b[i] = [l, r];
    }
    Some(a)
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut weights = HashMap::new();
    for line in data.split('\n').filter(|l| !l.is_empty()) {
        let mut new_weights = weights.clone();
        let (kind, coordinates) = line.split_once(' ').unwrap();
        let mut ranges = coordinates.split(',');
        let op_cube = [
            extract(ranges.next().unwrap()),
            extract(ranges.next().unwrap()),
            extract(ranges.next().unwrap()),
        ];
        for (cube, weight) in weights {
            if let Some(intersection) = intersection(cube, op_cube) {
                *new_weights.entry(intersection).or_insert(0) -= weight;
            }
        }
        if kind == "on" {
            *new_weights.entry(op_cube).or_insert(0) += 1;
        }
        weights = new_weights;
    }
    let total = weights
        .into_iter()
        .map(|(c, s)| c.into_iter().map(|d| d[1] - d[0] + 1).product::<i64>() * s)
        .sum::<i64>();
    println!("{}", total);
}
