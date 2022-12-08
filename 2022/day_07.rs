use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

#[derive(Debug, Default, Eq, PartialEq)]
enum FileKind {
    #[default]
    File,
    Dir,
}

#[derive(Debug, Default)]
struct File {
    kind: FileKind,
    size: usize,
    parent: Option<Rc<RefCell<File>>>,
    children: HashMap<String, Rc<RefCell<File>>>,
}

fn main() {
    let data = std::fs::read_to_string("input.txt").unwrap();
    dbg!(sum_of_sizes(data));
}

fn sum_of_sizes(data: String) -> usize {
    let mut files = VecDeque::new();
    let mut indegree = std::collections::HashMap::new();
    let root = Rc::new(RefCell::new(File {
        kind: FileKind::Dir,
        size: 0,
        parent: None,
        children: HashMap::new(),
    }));
    let mut node = Rc::clone(&root);
    for line in data.split('\n') {
        let tokens = line.split(' ').collect::<Vec<_>>();
        match tokens[0] {
            "$" => {
                if tokens[1] != "cd" {
                    continue;
                }
                node = match tokens[2] {
                    "/" => Rc::clone(&root),
                    ".." => {
                        if let Some(ref par) = node.borrow().parent {
                            Rc::clone(par)
                        } else {
                            Rc::clone(&root)
                        }
                    }
                    file => Rc::clone(&node.borrow().children[file]),
                };
            }
            "dir" => {
                node.borrow_mut().children.insert(
                    String::from(tokens[1]),
                    Rc::new(RefCell::new(File {
                        kind: FileKind::Dir,
                        size: 0,
                        parent: Some(Rc::clone(&node)),
                        children: HashMap::new(),
                    })),
                );
                *indegree.entry(node.as_ptr()).or_insert(0) += 1;
            }
            "" => {}
            size => {
                let file = Rc::new(RefCell::new(File {
                    kind: FileKind::File,
                    size: size.parse::<usize>().unwrap(),
                    parent: Some(Rc::clone(&node)),
                    children: HashMap::new(),
                }));
                node.borrow_mut()
                    .children
                    .insert(String::from(tokens[1]), Rc::clone(&file));
                *indegree.entry(node.as_ptr()).or_insert(0) += 1;
                files.push_back(file);
            }
        }
    }
    let mut sizes = vec![0];
    while let Some(file) = files.pop_front() {
        let file = file.borrow();
        if let Some(ref par) = file.parent {
            par.borrow_mut().size += file.size;
            *indegree.get_mut(&par.as_ptr()).unwrap() -= 1;
            if indegree[&par.as_ptr()] == 0 {
                files.push_back(Rc::clone(par));
            }
        }
        if file.kind == FileKind::Dir {
            sizes.push(file.size);
        }
    }
    sizes.sort();
    const REQUIRED: usize = 30000000;
    let unused_space = 70000000 - sizes[sizes.len() - 1];
    for x in sizes {
        if unused_space + x >= REQUIRED {
            return x;
        }
    }
    unreachable!()
}
