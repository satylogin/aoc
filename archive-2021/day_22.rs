use std::cmp::{max, min};

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

fn remove(mut from: [[i64; 2]; 3], to_remove: [[i64; 2]; 3]) -> Vec<[[i64; 2]; 3]> {
    let mut slices = vec![];
    if let Some(inter) = intersection(from, to_remove) {
        for i in 0..3 {
            let mut left = from;
            if left[i][0] < inter[i][0] {
                left[i][1] = inter[i][0] - 1;
                slices.push(left);
                from[i][0] = inter[i][0];
            }

            let mut right = from;
            if right[i][1] > inter[i][1] {
                right[i][0] = inter[i][1] + 1;
                slices.push(right);
                from[i][1] = inter[i][1];
            }
        }
    } else {
        slices.push(from);
    }
    slices
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();

    let mut cubes = vec![];
    for line in data.split('\n').filter(|l| !l.is_empty()) {
        let mut new_cubes = vec![];
        let (kind, coordinates) = line.split_once(' ').unwrap();
        let mut ranges = coordinates.split(',');
        let op_cube = [
            extract(ranges.next().unwrap()),
            extract(ranges.next().unwrap()),
            extract(ranges.next().unwrap()),
        ];
        for cube in cubes {
            new_cubes.append(&mut remove(cube, op_cube));
        }
        if kind == "on" {
            new_cubes.push(op_cube);
        }
        cubes = new_cubes;
    }
    let total = cubes
        .into_iter()
        .map(|c| c.into_iter().map(|d| d[1] - d[0] + 1).product::<i64>())
        .sum::<i64>();
    dbg!(total);
}
