use std::cmp::Ordering;
use std::collections::HashSet;
use std::iter::Iterator;

pub fn next_permutation<T: std::cmp::Ord>(arr: &mut [T]) -> bool {
    let last_ascending = if let Some(i) = arr.windows(2).rposition(|w| w[0] < w[1]) {
        i
    } else {
        arr.reverse();
        return false;
    };
    let swap_with = arr[last_ascending + 1..]
        .binary_search_by(|n| match arr[last_ascending].cmp(n) {
            Ordering::Equal => Ordering::Greater,
            ord => ord,
        })
        .unwrap_err();
    arr.swap(last_ascending, last_ascending + swap_with);
    arr[last_ascending + 1..].reverse();
    true
}

struct Orientation {
    points: Vec<[i32; 3]>,
    signs: [[i32; 3]; 8],
    sign_idx: usize,
    rotation: [usize; 3],
    rotation_count: usize,
}

impl Orientation {
    fn new(points: Vec<[i32; 3]>) -> Self {
        Self {
            points,
            signs: [
                [1, 1, 1],
                [1, 1, -1],
                [1, -1, 1],
                [1, -1, -1],
                [-1, 1, 1],
                [-1, 1, -1],
                [-1, -1, 1],
                [-1, -1, -1],
            ],
            sign_idx: 0,
            rotation: [0, 1, 2],
            rotation_count: 0,
        }
    }
}

impl Iterator for Orientation {
    type Item = Vec<[i32; 3]>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.sign_idx >= self.signs.len() {
            return None;
        }
        let sign = self.signs[self.sign_idx];
        let points = self
            .points
            .iter()
            .map(|p| {
                [
                    p[self.rotation[0]] * sign[0],
                    p[self.rotation[1]] * sign[1],
                    p[self.rotation[2]] * sign[2],
                ]
            })
            .collect();
        if self.rotation_count == 6 {
            self.rotation_count = 0;
            self.sign_idx += 1;
        }
        next_permutation(&mut self.rotation);
        self.rotation_count += 1;
        Some(points)
    }
}

fn relative_pos(source: &[i32], target: &[i32]) -> [i32; 3] {
    [
        target[0] - source[0],
        target[1] - source[1],
        target[2] - source[2],
    ]
}

fn resolve_pos(source: &[i32], target: &[i32]) -> [i32; 3] {
    [
        target[0] + source[0],
        target[1] + source[1],
        target[2] + source[2],
    ]
}

fn maybe_merge(
    scan_a: &Vec<[i32; 3]>,
    scan_b: &Vec<[i32; 3]>,
) -> Option<([i32; 3], Vec<[i32; 3]>)> {
    for i in 0..scan_a.len() {
        let pos_a = scan_a
            .iter()
            .map(|p| relative_pos(&scan_a[i], p))
            .collect::<HashSet<_>>();
        for j in 0..scan_b.len() {
            let pos_b = scan_b
                .iter()
                .map(|p| relative_pos(&scan_b[j], p))
                .collect::<Vec<_>>();
            let overlaps = pos_b
                .iter()
                .map(|p| pos_a.contains(p) as usize)
                .sum::<usize>();
            if overlaps >= 12 {
                let mut merged = scan_a.iter().map(|e| *e).collect::<HashSet<_>>();
                for p in &pos_b {
                    merged.insert(resolve_pos(&scan_a[i], p));
                }
                let scanner_b_pos = relative_pos(&scan_b[j], &scan_a[i]);
                return Some((scanner_b_pos, merged.into_iter().collect()));
            }
        }
    }
    None
}

fn main() {
    let mut scans = vec![];
    let mut scan = vec![];
    let parse = |x: &str| x.parse::<i32>().unwrap();
    for line in std::fs::read_to_string("input.txt").unwrap().split('\n') {
        if line.is_empty() {
            scans.push(Some(scan));
            scan = vec![];
        } else if line.contains("scan") {
            // nothing to do
        } else {
            let mut p = line.split(',');
            scan.push([
                parse(p.next().unwrap()),
                parse(p.next().unwrap()),
                parse(p.next().unwrap()),
            ]);
        }
    }
    let mut m_count = 1;
    let mut map = scans[0].take().unwrap();
    let mut scanners = vec![[0, 0, 0]];
    while scans.iter().map(|s| s.is_some() as usize).sum::<usize>() > 0 {
        for i in 1..scans.len() {
            if let Some(other) = scans[i].clone() {
                for o in Orientation::new(other.clone()) {
                    let res = maybe_merge(&map, &o);
                    if res.is_some() {
                        m_count += 1;
                        let (scanner_pos, m) = res.unwrap();
                        map = m;
                        scanners.push(scanner_pos);
                        scans[i] = None;
                        break;
                    }
                }
            }
        }
        dbg!(m_count);
    }
    let mut max_dist = 0;
    for i in 0..scanners.len() {
        for j in i + 1..scanners.len() {
            max_dist = std::cmp::max(
                (scanners[i][0] - scanners[j][0]).abs()
                    + (scanners[i][1] - scanners[j][1]).abs()
                    + (scanners[i][2] - scanners[j][2]).abs(),
                max_dist,
            );
        }
    }
    println!("beacons: {}, max scanner dist: {}", map.len(), max_dist);
}
