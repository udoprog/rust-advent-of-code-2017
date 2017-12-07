use std::io::{BufRead, BufReader, Read};
use failure::Error;
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Default)]
struct Node {
    id: String,
    weight: i32,
    parent: Option<String>,
    children: Vec<String>,
}

#[derive(Debug)]
struct Tree {
    nodes: HashMap<String, Node>,
}

impl Tree {
    /// Calculate the subtree weight for the given node.
    pub fn subtree_weight(&self, node: &Node) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(node);

        let mut weight = 0i32;

        while let Some(node) = queue.pop_front() {
            weight += node.weight;

            for c in &node.children {
                queue.push_back(self.nodes.get(c.as_str()).expect("child"));
            }
        }

        weight
    }

    /// Find the root node.
    pub fn find_root(&self) -> Option<&Node> {
        for node in self.nodes.values() {
            if node.parent.is_none() {
                return Some(node);
            }
        }

        None
    }

    /// Find the outlier child node.
    ///
    /// This assumes all child nodes have the same weight except the outlier.
    pub fn find_outlier(&self, node: &Node) -> Option<(&Node, i32)> {
        let mut weights: HashMap<i32, Vec<&Node>> = HashMap::new();

        for c in &node.children {
            let c = self.nodes.get(c.as_str()).expect("node");

            weights
                .entry(self.subtree_weight(c))
                .or_insert_with(Vec::new)
                .push(c);
        }

        let mut weights = weights.into_iter().collect::<Vec<_>>();
        weights.sort_by(|a, b| a.1.len().cmp(&b.1.len()));

        let mut w = weights.into_iter();
        let first = w.next().expect("first");

        if let Some(second) = w.next() {
            let node = *first.1.iter().next().expect("node");
            let diff = second.0 - first.0;

            Some((node, diff))
        } else {
            None
        }
    }
}

pub fn run<R: Read>(reader: R) -> Result<(String, i32), Error> {
    let mut nodes: HashMap<String, Node> = HashMap::new();

    for line in BufReader::new(reader).lines() {
        let line = line?;

        let mut line = line.trim().split(" -> ");
        let mut left = line.next().expect("lhs").split(" ");

        let name = left.next().expect("a name");
        let number = left.next().expect("a number");
        let number: i32 = number[1..number.len() - 1].parse()?;

        let targets: Vec<&str> = line.next().iter().flat_map(|t| t.split(", ")).collect();

        for t in &targets {
            let n = nodes.entry(t.to_string()).or_insert_with(Node::default);
            n.parent = Some(name.to_string());
        }

        let n = nodes.entry(name.to_string()).or_insert_with(Node::default);
        n.id = name.to_string();
        n.weight = number;
        n.children.extend(
            targets.into_iter().map(|s| s.to_string()),
        );
    }

    let tree = Tree { nodes: nodes };

    let root = tree.find_root().expect("root node");

    let mut node = root;
    let mut diff = 0i32;

    while let Some((n, d)) = tree.find_outlier(node) {
        node = n;
        diff = d;
    }

    Ok((root.id.to_string(), node.weight + diff))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day7.txt");

    #[test]
    fn test_all() {
        assert_eq!(
            ("veboyvy".to_string(), 749),
            run(Cursor::new(INPUT)).unwrap()
        );
    }
}
