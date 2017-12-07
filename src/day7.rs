use std::io::{BufRead, BufReader, Read};
use failure::Error;
use std::collections::{HashMap, VecDeque};

struct Tree {
    pub parents: HashMap<String, String>,
    pub children: HashMap<String, Vec<String>>,
    pub nodes: HashMap<String, i32>,
}

impl Tree {
    /// Calculate the subtree weight for the given node.
    pub fn subtree_weight(&self, node: &str) -> i32 {
        let mut queue = VecDeque::new();
        queue.push_back(node);

        let mut weight = 0i32;

        while let Some(node) = queue.pop_front() {
            weight += self.nodes.get(node).map(|v| *v).unwrap_or(0i32);

            if let Some(children) = self.children.get(node) {
                queue.extend(children.iter().map(|s| s.as_str()));
            }
        }

        weight
    }

    /// Find the root node.
    pub fn find_root(&self) -> Option<&str> {
        for node in self.nodes.keys() {
            if self.parents.get(node.as_str()).is_none() {
                return Some(node);
            }
        }

        None
    }

    /// Find all child nodes for the given node.
    pub fn children(&self, node: &str) -> Vec<&str> {
        let mut children = Vec::new();

        if let Some(c) = self.children.get(node) {
            children.extend(c.iter().map(|s| s.as_str()));
        }

        children
    }

    /// Find the weight for the given node.
    pub fn weight(&self, node: &str) -> Option<i32> {
        self.nodes.get(node).map(|v| *v)
    }

    /// Find the outlier child node.
    ///
    /// This assumes all child nodes have the same weight except the outlier.
    pub fn find_outlier(&self, node: &str) -> Option<(&str, i32)> {
        let mut weights: HashMap<i32, Vec<&str>> = HashMap::new();

        for c in self.children(node) {
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
    let mut data = String::new();
    let mut reader = BufReader::new(reader);

    let mut parents = HashMap::new();
    let mut children = HashMap::new();
    let mut nodes = HashMap::new();

    while reader.read_line(&mut data)? > 0 {
        {
            let mut line = data.trim().split(" -> ");
            let mut left = line.next().expect("lhs").split(" ");

            let name = left.next().expect("a name");
            let number = left.next().expect("a number");
            let number: i32 = number[1..number.len() - 1].parse()?;

            let targets = if let Some(targets) = line.next() {
                targets.split(", ").collect::<Vec<_>>()
            } else {
                vec![]
            };

            nodes.insert(name.to_string(), number);

            for target in targets {
                parents.insert(target.to_string(), name.to_string());
                children
                    .entry(name.to_string())
                    .or_insert_with(Vec::new)
                    .push(target.to_string());
            }
        }

        data.clear();
    }

    let tree = Tree {
        parents: parents,
        children: children,
        nodes: nodes,
    };

    let root = tree.find_root().expect("root node");

    let mut node = root;
    let mut diff = 0i32;

    while let Some((n, d)) = tree.find_outlier(node) {
        node = n;
        diff = d;
    }

    let actual = tree.weight(node).expect("node weight");
    Ok((root.to_string(), actual + diff))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day7.txt");

    #[test]
    fn test_part1() {
        assert_eq!(
            ("veboyvy".to_string(), 749),
            run(Cursor::new(INPUT)).unwrap()
        );
    }
}
