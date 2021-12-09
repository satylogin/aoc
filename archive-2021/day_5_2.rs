fn to_usize(d: Option<&str>) -> usize {
    d.unwrap().parse::<usize>().unwrap()
}

struct Traverse {
    start: (usize, usize),
    end: (usize, usize),
    done: bool,
}

impl Traverse {
    fn new(a: &[usize], b: &[usize]) -> Self {
        Self {
            start: (a[0], a[1]),
            end: (b[0], b[1]),
            done: false,
        }
    }

    fn dir(a: usize, b: usize) -> i32 {
        if a < b {
            1
        } else if a > b {
            -1
        } else {
            0
        }
    }

    fn step(&mut self) -> Option<(usize, usize)> {
        if self.done {
            return None;
        }
        let ret = self.start.clone();
        if self.start == self.end {
            self.done = true;
        } else {
            self.start.0 = (self.start.0 as i32 + Self::dir(self.start.0, self.end.0)) as usize;
            self.start.1 = (self.start.1 as i32 + Self::dir(self.start.1, self.end.1)) as usize;
        }
        Some(ret)
    }
}

fn main() {
    let lines: Vec<[[usize; 2]; 2]> = std::fs::read_to_string("input.txt")
        .unwrap()
        .split('\n')
        .map(|l| {
            let mut iter = l.split(" -> ");
            let mut left = iter.next().unwrap().split(',');
            let mut right = iter.next().unwrap().split(',');
            [
                [to_usize(left.next()), to_usize(left.next())],
                [to_usize(right.next()), to_usize(right.next())],
            ]
        })
        .collect();

    let mut grid = vec![vec![0; 1000]; 1000];
    for line in &lines {
        let mut t = Traverse::new(&line[0], &line[1]);
        while let Some((x, y)) = t.step() {
            grid[x][y] += 1;
        }
    }
    dbg!(grid
        .iter()
        .map(|r| r.iter().map(|c| (*c > 1) as usize).sum::<usize>())
        .sum::<usize>());
}
