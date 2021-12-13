use std::collections::{BTreeMap, BTreeSet};

struct Graph<'l> {
    children: BTreeMap<&'l str, BTreeSet<&'l str>>,
}

impl<'l> Graph<'l> {
    fn count_paths(&self, node: &str, visited: &mut BTreeSet<&'l str>, budget: bool) -> usize {
        if node == "end" {
            return 1;
        }
        let mut paths = 0;
        for &child in &self.children[node] {
            if child.chars().all(char::is_uppercase) {
                paths += self.count_paths(child, visited, budget);
            } else if !visited.contains(child) {
                visited.insert(child);
                paths += self.count_paths(child, visited, budget);
                visited.remove(child);
            } else if child != "start" && budget {
                paths += self.count_paths(child, visited, false);
            }
        }
        paths
    }
}

fn main() {
    let now = std::time::Instant::now();
    let data: String = std::fs::read_to_string("input.txt").unwrap();

    let mut children = BTreeMap::new();
    data.split('\n').filter(|l| !l.is_empty()).for_each(|l| {
        let mut splitter = l.split('-');
        let a = splitter.next().unwrap();
        let b = splitter.next().unwrap();
        children.entry(a).or_insert(BTreeSet::new()).insert(b);
        children.entry(b).or_insert(BTreeSet::new()).insert(a);
    });
    let mut visited = BTreeSet::new();
    visited.insert("start");
    let traverse = Graph { children };
    println!("{}", traverse.count_paths("start", &mut visited, true));
    println!("elapsed: {}", now.elapsed().as_millis());
}
