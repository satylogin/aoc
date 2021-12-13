fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let mut lines = data.split('\n');

    let mut grid = vec![vec!['.'; 2000]; 2000];
    while let Some(line) = lines.next() {
        if line.is_empty() {
            break;
        }
        let mut splitter = line.split(',');
        let x = splitter.next().unwrap().parse::<usize>().unwrap();
        let y = splitter.next().unwrap().parse::<usize>().unwrap();
        grid[y][x] = '#';
    }

    while let Some(line) = lines.next() {
        if line.is_empty() {
            continue;
        }
        let action = line.split(' ').nth(2).unwrap();
        let mut splitter = action.split('=');
        let axis = splitter.next().unwrap();
        let point = splitter.next().unwrap().parse::<usize>().unwrap();
        if axis == "x" {
            (0..2000).for_each(|i| grid[i][point] = '|');
            if point == 0 || point == 1999 {
                continue;
            }
            let mut a = point - 1;
            let mut b = point + 1;
            loop {
                (0..2000).for_each(|i| {
                    if grid[i][b] == '#' {
                        grid[i][a] = '#';
                    }
                    grid[i][b] = '.';
                });
                if a == 0 || b == 1999 {
                    break;
                }
                a -= 1;
                b += 1;
            }
        } else {
            (0..2000).for_each(|j| grid[point][j] = '-');
            if point == 0 || point == 1999 {
                continue;
            }
            let mut a = point - 1;
            let mut b = point + 1;
            loop {
                (0..2000).for_each(|j| {
                    if grid[b][j] == '#' {
                        grid[a][j] = '#';
                    }
                    grid[b][j] = '.';
                });
                if a == 0 || b == 1999 {
                    break;
                }
                a -= 1;
                b += 1;
            }
        }
        let cnt = grid
            .iter()
            .map(|row| row.iter().map(|p| (*p == '#') as usize).sum::<usize>())
            .sum::<usize>();
        println!("count = {}", cnt);
        for i in 0..50 {
            for j in 0..50 {
                print!("{} ", grid[i][j]);
            }
            println!();
        }
    }
}
