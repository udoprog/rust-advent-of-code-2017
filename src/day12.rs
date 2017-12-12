use std::io::Read;
use std::collections::{HashMap, HashSet, VecDeque};
use failure::Error;

fn visited(graph: &HashMap<u64, Vec<u64>>, current: u64) -> HashSet<u64> {
    let mut visited = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back(current);

    while let Some(id) = queue.pop_front() {
        if visited.insert(id) {
            queue.extend(graph.get(&id).into_iter().flat_map(|v| v.iter().cloned()));
        }
    }

    visited
}

pub fn run<R: Read>(mut reader: R) -> Result<(u64, u64), Error> {
    let data = {
        let mut data = String::new();
        reader.read_to_string(&mut data)?;
        data
    };

    let mut graph: HashMap<u64, Vec<u64>> = HashMap::new();
    let mut all_ids = HashSet::new();

    for line in data.lines() {
        let mut it = line.split("<->");

        let left: u64 = it.next().expect("left side").trim().parse()?;

        let right: Vec<u64> = it.next()
            .expect("right side")
            .trim()
            .split(", ")
            .map(|s| s.parse().map_err(Into::into))
            .collect::<Result<Vec<u64>, Error>>()?;

        all_ids.insert(left);
        all_ids.extend(right.iter().cloned());

        graph.insert(left, right);
    }

    let zero_group = visited(&graph, 0).len() as u64;

    let mut groups = 0u64;

    while let Some(next_id) = all_ids.iter().cloned().next() {
        for found in visited(&graph, next_id) {
            all_ids.remove(&found);
        }

        groups += 1;
    }

    Ok((zero_group, groups))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    static INPUT: &str = include_str!("../input/day12.txt");

    #[test]
    fn problem() {
        assert_eq!(run(Cursor::new(INPUT)).unwrap(), (113, 202));
    }
}
