#![feature(map_first_last)]
#![feature(test)]

use std::collections::BTreeSet;
use std::iter::Iterator;

pub struct GridNeighbour {
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    offset: Vec<(i64, i64)>,
    offset_idx: usize,
}

impl GridNeighbour {
    pub fn new(x: usize, y: usize, n: usize, m: usize, diagonal_traversal: bool) -> Self {
        let mut offset = vec![(-1, 0), (1, 0), (0, -1), (0, 1)];
        if diagonal_traversal {
            offset.append(&mut vec![(-1, -1), (-1, 1), (1, -1), (1, 1)]);
        }
        Self {
            x,
            y,
            n,
            m,
            offset,
            offset_idx: 0,
        }
    }
}

impl Iterator for GridNeighbour {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while self.offset_idx < self.offset.len() {
            let x = self.x as i64 + self.offset[self.offset_idx].0;
            let y = self.y as i64 + self.offset[self.offset_idx].1;
            self.offset_idx += 1;
            if x >= 0 && x < self.n as i64 && y >= 0 && y < self.m as i64 {
                return Some((x as usize, y as usize));
            }
        }
        None
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let grid = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| {
            l.bytes()
                .map(|e| (e - '0' as u8) as usize)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let n = grid.len();
    let m = grid[0].len();
    let mut new_grid = vec![vec![0; m * 5]; n * 5];
    for i in 0..n * 5 {
        for j in 0..m * 5 {
            new_grid[i][j] = grid[i % n][j % n] + i / n + j / m;
            if new_grid[i][j] > 9 {
                new_grid[i][j] -= 9;
            }
        }
    }
    let grid = new_grid;
    let n = n * 5;
    let m = m * 5;

    let mut dist = vec![vec![usize::MAX; m]; n];

    let mut q = BTreeSet::new();
    dist[0][0] = 0;
    q.insert((dist[0][0], (0, 0)));
    while let Some((_, (x, y))) = q.pop_first() {
        let neighbours = GridNeighbour::new(x, y, n, m, false);
        for (a, b) in neighbours {
            if dist[a][b] > dist[x][y] + grid[a][b] {
                dist[a][b] = dist[x][y] + grid[a][b];
                q.insert((dist[a][b], (a, b)));
            }
        }
    }
    println!("{}", dist[n - 1][m - 1]);
}

extern crate test;
#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_main(b: &mut Bencher) {
        b.iter(|| main());
    }
}
