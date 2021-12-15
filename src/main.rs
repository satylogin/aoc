#![feature(map_first_last)]
#![feature(test)]

use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::iter::Iterator;

pub struct GridNeighbour {
    x: usize,
    y: usize,
    n: usize,
    m: usize,
    offset: Vec<(i32, i32)>,
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
            let x = self.x as i32 + self.offset[self.offset_idx].0;
            let y = self.y as i32 + self.offset[self.offset_idx].1;
            self.offset_idx += 1;
            if x >= 0 && x < self.n as i32 && y >= 0 && y < self.m as i32 {
                return Some((x as usize, y as usize));
            }
        }
        None
    }
}

fn input_grid() -> Vec<Vec<usize>> {
    std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| l.bytes().map(|e| (e - '0' as u8) as usize).collect())
        .collect()
}

fn expand_grid(grid: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
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
    new_grid
}

fn cost(grid: &Vec<Vec<usize>>) -> usize {
    let n = grid.len();
    let m = grid[0].len();
    let mut dist = vec![vec![usize::MAX; m]; n];
    let mut q = BinaryHeap::new();

    dist[0][0] = 0;
    q.push((Reverse(dist[0][0]), (0, 0)));
    while let Some((_, (x, y))) = q.pop() {
        let neighbours = GridNeighbour::new(x, y, n, m, false);
        for (a, b) in neighbours {
            if dist[a][b] > dist[x][y] + grid[a][b] {
                dist[a][b] = dist[x][y] + grid[a][b];
                q.push((Reverse(dist[a][b]), (a, b)));
            }
        }
    }

    dist[n - 1][m - 1]
}

fn main() {
    println!("{}", cost(&expand_grid(&input_grid())));
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

    #[bench]
    fn bench_input_grid(b: &mut Bencher) {
        b.iter(|| input_grid());
    }

    #[bench]
    fn bench_expand_grid(b: &mut Bencher) {
        let grid = input_grid();
        b.iter(|| expand_grid(&grid));
    }

    #[bench]
    fn bench_cost(b: &mut Bencher) {
        let grid = expand_grid(&input_grid());
        b.iter(|| cost(&grid));
    }
}
