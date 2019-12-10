use std::collections::{HashMap, HashSet};
use std::default::Default;

#[derive(Debug)]
enum Error {
    InvalidInput,
}

type Id = usize;

#[derive(Debug)]
struct Node {
    parent: Option<Id>,
    children: Vec<Id>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            parent: None,
            children: Vec::new(),
        }
    }
}

type Tree = HashMap<Id, Node>;

fn build_tree(input: &str) -> crate::Result<(HashMap<&str, Id>, Tree)> {
    let mut nodes = Tree::new();
    let mut ids = HashMap::new();
    let mut idc = 0;

    for l in input.lines() {
        let mut ns = l.split(')');
        let parent = ns
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
        let child = ns
            .next()
            .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

        let parent = *ids.entry(parent).or_insert_with(|| {
            idc += 1;
            idc - 1
        });
        let child = *ids.entry(child).or_insert_with(|| {
            idc += 1;
            idc - 1
        });

        nodes.entry(parent).or_default().children.push(child);
        nodes.entry(child).or_default().parent = Some(parent);
    }

    Ok((ids, nodes))
}

pub fn part1(input: &str) -> crate::Result<u32> {
    let (_, nodes) = build_tree(input)?;

    Ok(nodes.iter().try_fold(
        0,
        |mut acc, (_, mut node)| -> crate::Result<u32> {
            while let Some(n) = node.parent {
                acc += 1;
                node = nodes
                    .get(&n)
                    .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
            }
            Ok(acc)
        },
    )?)
}

pub fn part2(input: &str) -> crate::Result<u32> {
    let (ids, nodes) = build_tree(input)?;
    let santa = *ids
        .get("SAN")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;
    let me = *ids
        .get("YOU")
        .ok_or_else(|| crate::Error::boxed(Error::InvalidInput))?;

    let mut dist = 0;
    let mut frontier = vec![me];
    let mut visited: HashSet<_> = frontier.iter().cloned().collect();
    while !visited.contains(&santa) {
        dist += 1;

        frontier = frontier
            .into_iter()
            .flat_map(|id| {
                let n = &nodes[&id];
                if let Some(p) = n.parent {
                    vec![p]
                } else {
                    vec![]
                }
                .into_iter()
                .chain(n.children.iter().cloned())
                .filter(|id| !visited.contains(id))
            })
            .collect();

        visited.extend(&frontier);
    }

    Ok(if dist <= 2 { 0 } else { dist - 2 })
}
