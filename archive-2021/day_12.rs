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
    #[must_use]
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

fn steps_to_end(char_grid: Vec<Vec<char>>) -> usize {
    let (n, m) = (char_grid.len(), char_grid[0].len());
    let mut grid = vec![vec![0; m]; n];
    let (mut x, mut y) = (0, 0);
    for i in 0..n {
        for j in 0..m {
            grid[i][j] = match char_grid[i][j] {
                'S' => 0,
                'E' => {
                    (x, y) = (i, j);
                    25
                }
                c => (c as u8 - 'a' as u8) as i32,
            };
        }
    }
    let mut steps = vec![vec![-1; m]; n];
    steps[x][y] = 0;
    let mut pq = std::collections::BinaryHeap::new();
    pq.push((0, x, y));
    while let Some((_, x, y)) = pq.pop() {
        for (a, b) in GridNeighbour::new(x, y, n, m, false) {
            if (grid[x][y] - 1 == grid[a][b] || grid[a][b] >= grid[x][y]) && steps[a][b] == -1 {
                steps[a][b] = 1 + steps[x][y];
                pq.push((-steps[a][b], a, b));
            }
        }
    }
    let mut ans = i32::MAX;
    for i in 0..n {
        for j in 0..m {
            if grid[i][j] == 0 && steps[i][j] != -1 {
                ans = std::cmp::min(ans, steps[i][j]);
            }
        }
    }
    return ans as usize;
}

fn solve(input: &str) {
    let grid = std::fs::read_to_string(input)
        .unwrap()
        .split('\n')
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    println!("steps: {}", steps_to_end(grid));
}

fn main() {
    for input in ["test.txt", "input.txt"] {
        let now = std::time::Instant::now();
        solve(input);
        println!("elapsed: {}ms", now.elapsed().as_millis());
    }
}
