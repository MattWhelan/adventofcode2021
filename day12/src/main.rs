use std::collections::HashSet;
use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct Edge {
    left: String,
    right: String,
}

impl Edge {
    fn contains(&self, node: &str) -> bool {
        self.left == node || self.right == node
    }
}

impl FromStr for Edge {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split("-").map(|s| s.to_string()).collect_tuple().unwrap();
        Ok(Edge {
            left,
            right
        })
    }
}

fn main() -> Result<()> {
    let input: Vec<Edge> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    let paths = paths1(&input, "start", "end", HashSet::new());

    println!("Part 1: {}", paths.len());

    let paths2 = paths2(&input, "start", "end", HashSet::new(), false);

    // not 113558
    println!("Part 2: {}", paths2.len());


    Ok(())
}

fn paths1(edges: &[Edge], start: &str, end: &str, mut visited: HashSet<String>) -> Vec<Vec<String>> {
    if start == end {
        return vec![vec![start.to_string()]];
    }

    // small check
    if start == start.to_lowercase() {
        visited.insert(start.to_string());
    }

    let mut sub_paths: Vec<Vec<String>> = edges.iter()
        .filter(|e| e.contains(start))
        .map(|e| if e.left == start {
            &e.right
        } else {
            &e.left
        })
        .filter(|n| !visited.contains(*n))
        .flat_map(|n| paths1(edges, n, end, visited.clone()))
        .collect();

    for p in sub_paths.iter_mut() {
        p.insert(0, start.to_string());
    }

    sub_paths
}

fn paths2(edges: &[Edge], start: &str, end: &str, mut visited: HashSet<String>, revisited: bool) -> Vec<Vec<String>> {
    if start == end {
        return vec![vec![start.to_string()]];
    }

    // small check
    if start == start.to_lowercase() {
        visited.insert(start.to_string());
    }

    let mut sub_paths: Vec<Vec<String>> = edges.iter()
        .filter(|e| e.contains(start))
        .map(|e| if e.left == start {
            &e.right
        } else {
            &e.left
        })
        .flat_map(|n| {
            if !visited.contains(n) {
                paths2(edges, n, end, visited.clone(), revisited)
            } else if !revisited && n != "start" && n != "end" {
                paths2(edges, n, end, visited.clone(), true)
            } else {
                Vec::new()
            }
        })
        .collect();

    for p in sub_paths.iter_mut() {
        p.insert(0, start.to_string());
    }

    sub_paths
}

const TEST: &str = r#"fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW"#;
const INPUT: &str = r#"HF-qu
end-CF
CF-ae
vi-HF
vt-HF
qu-CF
hu-vt
CF-pk
CF-vi
qu-ae
ae-hu
HF-start
vt-end
ae-HF
end-vi
vi-vt
hu-start
start-ae
CS-hu
CF-vt
"#;
