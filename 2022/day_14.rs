const N: usize = 1000;

fn generate_grid(data: &str) -> Vec<Vec<char>> {
    let paths = data
        .split('\n')
        .map(move |l| {
            l.split(" -> ")
                .map(|p| {
                    let (x, y) = p.split_once(',').unwrap();
                    (x.parse::<i32>().unwrap(), y.parse::<i32>().unwrap())
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let max_y = paths
        .iter()
        .map(|p| p.iter().map(|(_, y)| *y).max().unwrap())
        .max()
        .unwrap() as usize
        + 2;
    let mut grid = vec![vec!['.'; N + 1]; max_y + 1];
    grid[max_y] = vec!['#'; N + 1];
    for path in paths {
        for i in 1..path.len() {
            let (mut x, mut y) = path[i - 1];
            grid[y as usize][x as usize] = '#';
            let (a, b) = path[i];
            let dx = if x == a { 0 } else { (a - x) / (a - x).abs() };
            let dy = if y == b { 0 } else { (b - y) / (b - y).abs() };
            loop {
                x += dx;
                y += dy;
                grid[y as usize][x as usize] = '#';
                if (x, y) == path[i] {
                    break;
                }
            }
        }
    }
    grid
}

fn solve(data: &str) {
    let mut grid = generate_grid(data);
    for round in 1.. {
        let (mut x, mut y) = (500, 0);
        loop {
            let mut found = false;
            for dx in [0, -1, 1] {
                let a = x + dx;
                if grid[y + 1][a as usize] == '.' {
                    (x, y) = (a, y + 1);
                    found = true;
                    break;
                }
            }
            if !found {
                grid[y][x as usize] = 'o';
                break;
            }
        }
        if (x, y) == (500, 0) {
            println!("abyss time: {}", round);
            break;
        }
    }
}

fn main() {
    for file in ["test.txt", "input.txt"] {
        let now = std::time::Instant::now();
        solve(&std::fs::read_to_string(file).unwrap());
        println!("elapsed: {}ms", now.elapsed().as_millis());
    }
}
