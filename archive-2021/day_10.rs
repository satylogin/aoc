fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split('\n').filter(|l| !l.is_empty());

    let fine: std::collections::HashMap<char, usize> = vec![(')', 1), (']', 2), ('}', 3), ('>', 4)]
        .into_iter()
        .collect();

    let matching: std::collections::HashMap<char, char> =
        vec![('(', ')'), ('[', ']'), ('{', '}'), ('<', '>')]
            .into_iter()
            .collect();

    let mut scores = vec![];
    for line in lines {
        let mut stack = vec![];
        let mut valid = true;
        for c in line.chars() {
            if vec!['(', '{', '[', '<'].contains(&c) {
                stack.push(c);
            } else {
                if stack.is_empty() || matching[stack.iter().last().unwrap()] != c {
                    valid = false;
                    break;
                } else {
                    stack.pop();
                }
            }
        }
        if valid {
            let mut cost = 0;
            for c in stack.into_iter().rev() {
                cost = cost * 5 + fine[&matching[&c]];
            }
            scores.push(cost);
        }
    }
    scores.sort();
    println!("{}", scores[scores.len() / 2]);
}
