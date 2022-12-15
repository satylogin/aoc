use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
enum Node {
    Leaf(i32),
    Inner(Box<Vec<Node>>),
}

fn parse(packet: &str) -> Node {
    let mut num = None;
    let mut vecs = vec![vec![]];
    let push_num = |num: Option<i32>, vecs: &mut Vec<Vec<Node>>| {
        let lev = vecs.len() - 1;
        if let Some(num) = num {
            vecs[lev].push(Node::Leaf(num));
        }
        None
    };
    for c in packet.chars() {
        match c {
            '[' => vecs.push(vec![]),
            ']' => {
                num = push_num(num, &mut vecs);
                let lev = vecs.len() - 1;
                let v = vecs.pop().unwrap();
                vecs[lev - 1].push(Node::Inner(Box::new(v)));
            }
            ' ' => {}
            ',' => num = push_num(num, &mut vecs),
            d => num = Some(num.unwrap_or(0) * 10 + (d as u8 - '0' as u8) as i32),
        };
    }
    Node::Inner(Box::new(vecs.pop().unwrap()))
}

fn compare(first: &Node, second: &Node) -> Ordering {
    match (first, second) {
        (Node::Leaf(f_val), Node::Leaf(s_val)) => (*f_val).cmp(s_val),
        (Node::Inner(f_list), Node::Inner(s_list)) => {
            let mut i = 0;
            while i < f_list.len() && i < s_list.len() {
                match compare(&f_list[i], &s_list[i]) {
                    Ordering::Equal => {}
                    other => return other,
                };
                i += 1;
            }
            f_list.len().cmp(&s_list.len())
        }
        (l, Node::Leaf(v)) => compare(l, &Node::Inner(Box::new(vec![Node::Leaf(*v)]))),
        (Node::Leaf(v), l) => compare(&Node::Inner(Box::new(vec![Node::Leaf(*v)])), l),
    }
}

fn solve(data: &str) {
    let mut nodes = data
        .split('\n')
        .filter(|l| !l.is_empty())
        .map(|l| parse(l))
        .collect::<Vec<_>>();
    nodes.append(&mut vec![parse("[[2]]"), parse("[[6]]")]);
    nodes.sort_by(|x, y| compare(x, y));
    let divider_nodes = vec![parse("[[2]]"), parse("[[6]]")];
    let decoder_key = (0..nodes.len())
        .filter(|&i| divider_nodes.contains(&nodes[i]))
        .map(|i| i + 1)
        .product::<usize>();
    println!("decoder_key: {}", decoder_key);
}

fn main() {
    for file in ["test.txt", "input.txt"] {
        let now = std::time::Instant::now();
        let data = std::fs::read_to_string(file).unwrap();
        solve(&data);
        println!("elapsed: {}ms", now.elapsed().as_millis());
    }
}
