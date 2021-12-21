struct NDIterator<const N: usize> {
    limits: [usize; N],
    current: [usize; N],
}

impl<const N: usize> NDIterator<N> {
    fn new(limits: [usize; N]) -> Self {
        Self {
            limits,
            current: [0; N],
        }
    }
}

impl<const N: usize> std::iter::Iterator for NDIterator<N> {
    type Item = [usize; N];

    fn next(&mut self) -> Option<Self::Item> {
        if self.current[0] == self.limits[0] {
            return None;
        }
        let to_return = self.current.clone();
        for i in (0..N).rev() {
            self.current[i] += 1;
            if self.current[i] < self.limits[i] {
                break;
            }
            if i != 0 {
                self.current[i] = 0;
            }
        }
        Some(to_return)
    }
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = data.split('\n').filter(|l| !l.is_empty());

    let resolver: Vec<char> = lines.next().unwrap().chars().collect();

    let mut grid: Vec<Vec<char>> = lines.map(|l| l.chars().collect()).collect();
    for i in 0..grid.len() {
        grid[i] = [&['.'; 51], &grid[i][..], &['.'; 51]].concat();
    }
    let r: Vec<Vec<char>> = (0..51).map(|_| vec!['.'; grid[0].len()]).collect();
    grid = [&r[..], &grid[..], &r[..]].concat();

    let n = grid.len();
    let m = grid[0].len();
    let mut resolved = vec![vec!['.'; m]; n];

    let valid = |x: i32, y: i32| x >= 0 && x < n as i32 && y >= 0 && y < m as i32;
    for s in 0..50 {
        for [i, j] in NDIterator::new([n, m]) {
            let num = NDIterator::new([3, 3])
                .map(|[x, y]| {
                    let (x, y) = (i as i32 + x as i32 - 1, j as i32 + y as i32 - 1);
                    if valid(x, y) && grid[x as usize][y as usize] == '#' {
                        '1'
                    } else {
                        '0'
                    }
                })
                .collect::<String>();
            resolved[i][j] = resolver[usize::from_str_radix(&num, 2).unwrap()];
        }
        std::mem::swap(&mut grid, &mut resolved);
        if s & 1 == 1 {
            for i in 1..n {
                grid[i][0] = '.';
                grid[i][m - 1] = '.';
            }
            for j in 1..m {
                grid[0][j] = '.';
                grid[n - 1][j] = '.';
            }
        }
    }
    let cnt = grid
        .iter()
        .map(|row| row.iter().map(|c| (*c == '#') as usize).sum::<usize>())
        .sum::<usize>();
    println!("{}", cnt);
}
