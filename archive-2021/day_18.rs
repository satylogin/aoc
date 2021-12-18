#[derive(Default, Debug)]
struct Node {
    data: i64,
    parent: usize,
    leaf: bool,
    child: [usize; 2],
}

struct Vault {
    nodes: Vec<Node>,
}

impl Vault {
    fn new() -> Self {
        Self {
            nodes: vec![Node::default()],
        }
    }

    fn allocate(&mut self) -> usize {
        self.nodes.push(Node::default());
        self.nodes.len() - 1
    }

    fn allocate_with(&mut self, data: i64, parent: usize, leaf: bool, child: [usize; 2]) -> usize {
        self.nodes.push(Node {
            data,
            parent,
            leaf,
            child,
        });
        self.nodes.len() - 1
    }

    fn get(&mut self, idx: usize) -> &mut Node {
        &mut self.nodes[idx]
    }
}

fn split(id: usize, vault: &mut Vault) -> bool {
    if vault.get(id).leaf {
        if vault.get(id).data < 10 {
            return false;
        }
        let data = vault.get(id).data;
        let cid_1 = vault.allocate_with(data / 2, id, true, [0, 0]);
        let cid_2 = vault.allocate_with(data - data / 2, id, true, [0, 0]);
        let parent = vault.get(id).parent;
        *vault.get(id) = Node {
            data: 0,
            parent,
            leaf: false,
            child: [cid_1, cid_2],
        };
        true
    } else {
        split(vault.get(id).child[0], vault) || split(vault.get(id).child[1], vault)
    }
}

fn explode(id: usize, depth: usize, vault: &mut Vault) -> bool {
    if vault.get(id).leaf {
        return false;
    }
    if explode(vault.get(id).child[0], depth + 1, vault)
        || explode(vault.get(id).child[1], depth + 1, vault)
    {
        return true;
    }
    if depth < 4 {
        return false;
    }
    for i in [0, 1] {
        let cid = vault.get(id).child[i];
        let data = vault.get(cid).data;
        let mut id = id;
        let mut pid = vault.get(id).parent;
        while pid != 0 && vault.get(pid).child[i] == id {
            id = pid;
            pid = vault.get(pid).parent;
        }
        if pid != 0 {
            id = vault.get(pid).child[i];
            while !vault.get(id).leaf {
                id = vault.get(id).child[1 - i];
            }
            vault.get(id).data += data;
        }
    }
    vault.get(id).leaf = true;
    vault.get(id).data = 0;
    true
}

fn magnitude(id: usize, vault: &mut Vault) -> i64 {
    if vault.get(id).leaf {
        vault.get(id).data
    } else {
        3 * magnitude(vault.get(id).child[0], vault) + 2 * magnitude(vault.get(id).child[1], vault)
    }
}

fn parse(line: &str, vault: &mut Vault) -> usize {
    let mut id = vault.allocate();
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '[' => {
                let nid = vault.allocate();
                vault.get(nid).parent = id;
                vault.get(id).child[0] = nid;
                stack.push(id);
                id = nid;
            }
            ',' => {
                let nid = vault.allocate();
                let pid = vault.get(id).parent;
                vault.get(pid).child[1] = nid;
                vault.get(nid).parent = pid;
                id = nid;
            }
            ']' => {
                id = stack.pop().unwrap();
            }
            d => {
                vault.get(id).data = (d as u8 - '0' as u8) as i64;
                vault.get(id).leaf = true;
            }
        }
    }
    id
}

fn join(left: usize, right: usize, vault: &mut Vault) -> usize {
    let pid = vault.allocate();
    vault.get(pid).child = [left, right];
    vault.get(left).parent = pid;
    vault.get(right).parent = pid;
    while explode(pid, 0, vault) || split(pid, vault) {}
    pid
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split('\n').collect::<Vec<_>>();
    let mut max_magnitude = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let mut vault = Vault::new();
            let id = join(
                parse(lines[i], &mut vault),
                parse(lines[j], &mut vault),
                &mut vault,
            );
            max_magnitude = std::cmp::max(max_magnitude, magnitude(id, &mut vault));
        }
    }
    println!("{}", max_magnitude);
}
