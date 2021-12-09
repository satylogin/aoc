use std::io::prelude::BufRead;

fn has_won(board: &Vec<Vec<Option<usize>>>) -> bool {
    (0..5).any(|i| (0..5).all(|j| board[i][j].is_none()))
        || (0..5).any(|j| (0..5).all(|i| board[i][j].is_none()))
}

fn mark(board: &mut Vec<Vec<Option<usize>>>, num: usize) {
    for i in 0..5 {
        for j in 0..5 {
            if board[i][j].is_some() && board[i][j].unwrap() == num {
                board[i][j] = None;
                return;
            }
        }
    }
}

fn main() {
    let f = std::fs::File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(f);

    let mut iter = reader.lines().enumerate();

    let nums = iter
        .next()
        .unwrap()
        .1
        .unwrap()
        .split(',')
        .map(|e| match e.parse::<usize>() {
            Ok(r) => Some(r),
            Err(_) => None,
        })
        .collect::<Vec<_>>();

    let mut boards = vec![];
    while let Some(_) = iter.next() {
        let board: Vec<Vec<Option<usize>>> = (0..5)
            .map(|_| {
                iter.next()
                    .unwrap()
                    .1
                    .unwrap()
                    .split_ascii_whitespace()
                    .map(|e| match e.parse::<usize>() {
                        Ok(r) => Some(r),
                        Err(_) => None,
                    })
                    .collect()
            })
            .collect();
        boards.push(board);
    }

    for num in nums {
        let num = num.unwrap();
        for board in &mut boards {
            mark(board, num);
            if has_won(board) {
                // calc
                let total: usize = board
                    .iter()
                    .map(|r| r.iter().map(|e| e.unwrap_or(0)).sum::<usize>())
                    .sum();
                println!("{}", total * num);
                return;
            }
        }
    }
}
