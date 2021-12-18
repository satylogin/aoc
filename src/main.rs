struct Node {
    data: i64,
    parent: *mut Node,
    is_leaf: bool,
    child: [*mut Node; 2],
}

fn g_node(data: i64, parent: *mut Node, is_leaf: bool, child: [*mut Node; 2]) -> *mut Node {
    Box::into_raw(Box::new(Node {
        data,
        parent,
        is_leaf,
        child,
    }))
}

fn split(node: *mut Node) -> bool {
    if unsafe { &*node }.is_leaf {
        let node_ref = unsafe { &mut *node };
        if node_ref.data < 10 {
            return false;
        }
        let data = node_ref.data;
        node_ref.is_leaf = false;
        node_ref.child = [
            g_node(data / 2, node, true, [std::ptr::null_mut(); 2]),
            g_node(data - data / 2, node, true, [std::ptr::null_mut(); 2]),
        ];
        true
    } else {
        split(unsafe { &*node }.child[0]) || split(unsafe { &*node }.child[1])
    }
}

fn explode(node: *mut Node, depth: usize) -> bool {
    let node_ref = unsafe { &*node };
    if node_ref.is_leaf {
        return false;
    }
    if explode(node_ref.child[0], depth + 1) || explode(node_ref.child[1], depth + 1) {
        return true;
    }
    if depth < 4 {
        return false;
    }
    for i in [0, 1] {
        let data = unsafe { &*node_ref.child[i] }.data;
        let mut node = node;
        let mut parent = unsafe { &*node }.parent;
        while !parent.is_null() && unsafe { &*parent }.child[i] == node {
            node = parent;
            parent = unsafe { &*node }.parent;
        }
        if !parent.is_null() {
            node = unsafe { &*parent }.child[i];
            while !unsafe { &*node }.is_leaf {
                node = unsafe { &*node }.child[1 - i];
            }
            unsafe { &mut *node }.data += data;
        }
    }
    unsafe { &mut *node }.is_leaf = true;
    unsafe { &mut *node }.data = 0;
    true
}

fn magnitude(node: &Node) -> i64 {
    if node.is_leaf {
        node.data
    } else {
        3 * magnitude(unsafe { &*node.child[0] }) + 2 * magnitude(unsafe { &*node.child[1] })
    }
}

fn parse(line: &str) -> *mut Node {
    let mut root = g_node(0, std::ptr::null_mut(), false, [std::ptr::null_mut(); 2]);
    let mut stack = vec![];
    for c in line.chars() {
        match c {
            '[' => {
                unsafe { &mut *root }.child[0] = g_node(0, root, false, [std::ptr::null_mut(); 2]);
                stack.push(root);
                root = unsafe { &mut *root }.child[0];
            }
            ',' => {
                let p = unsafe { &*root }.parent;
                unsafe { &mut *p }.child[1] = g_node(0, p, false, [std::ptr::null_mut(); 2]);
                root = unsafe { &mut *p }.child[1];
            }
            ']' => root = stack.pop().unwrap(),
            d => {
                unsafe { &mut *root }.data = (d as u8 - '0' as u8) as i64;
                unsafe { &mut *root }.is_leaf = true;
            }
        }
    }
    root
}

fn join(left: *mut Node, right: *mut Node) -> *mut Node {
    let node = g_node(0, std::ptr::null_mut(), false, [left, right]);
    unsafe { &mut *left }.parent = node;
    unsafe { &mut *right }.parent = node;
    while explode(node, 0) || split(node) {}
    node
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    let lines = data.split('\n').collect::<Vec<_>>();
    let mut max_mag = 0;
    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            let node = join(parse(lines[i]), parse(lines[j]));
            max_mag = std::cmp::max(max_mag, magnitude(unsafe { &*node }));
        }
    }
    println!("{}", max_mag);
}
