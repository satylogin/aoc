use std::collections::{BTreeMap, BTreeSet};

struct Traverse<'l> {
    children: BTreeMap<&'l str, BTreeSet<&'l str>>,
    visited: BTreeSet<&'l str>,
    special: bool,
    paths: usize,
}

impl<'l> Traverse<'l> {
    fn traverse(&mut self, node: &str) {
        if node == "end" {
            self.paths += 1;
            return;
        }
        for child in self.children[node].clone() {
            if child.chars().all(char::is_uppercase) {
                self.traverse(child);
            } else if !self.visited.contains(child) {
                self.visited.insert(child);
                self.traverse(child);
                self.visited.remove(child);
            } else if child != "start" && !self.special {
                self.special = true;
                self.traverse(child);
                self.special = false;
            }
        }
    }
}

fn main() {
    let mut children = BTreeMap::new();
    let data: String = std::fs::read_to_string("input.txt").unwrap();

    data.split('\n').filter(|l| !l.is_empty()).for_each(|l| {
        let mut splitter = l.split('-');
        let a = splitter.next().unwrap();
        let b = splitter.next().unwrap();
        children.entry(a).or_insert(BTreeSet::new()).insert(b);
        children.entry(b).or_insert(BTreeSet::new()).insert(a);
    });
    let mut visited = BTreeSet::new();
    visited.insert("start");
    let mut traverse = Traverse {
        children,
        visited,
        special: false,
        paths: 0,
    };
    traverse.traverse("start");
    println!("{}", traverse.paths);
}
