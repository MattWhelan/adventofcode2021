use std::str::FromStr;

use anyhow::Result;
use itertools::Itertools;

#[cfg(not(target_env = "msvc"))]
use jemallocator::Jemalloc;

#[cfg(not(target_env = "msvc"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

#[derive(Debug)]
struct Edge {
    left: String,
    right: String,
}

impl Edge {
    fn map(&self, node: &str) -> Option<&str> {
        if self.left == node {
            Some(&self.right)
        } else if self.right == node {
            Some(&self.left)
        } else {
            None
        }
    }
}

impl FromStr for Edge {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split("-").map(|s| s.to_string()).collect_tuple().unwrap();
        Ok(Edge { left, right })
    }
}

fn main() -> Result<()> {
    let input: Vec<Edge> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

    // let paths = paths1(&input, "start", "end", HashSet::new());
    // println!("Part 1: {}", paths.len());

    let paths2 = paths2(&input, "start", "end", Vec::new(), false);
    println!("Part 2: {}", paths2.len());

    Ok(())
}

fn is_lower(s: &str) -> bool {
    s.chars().all(|ch| ch.is_lowercase())
}

fn paths2<'a>(
    edges: &'a [Edge],
    start: &'a str,
    end: &str,
    mut visited: Vec<&'a str>,
    revisited: bool,
) -> Vec<Vec<&'a str>> {
    visited.push(start);

    if start == end {
        return vec![visited];
    }

    let sub_paths: Vec<Vec<&str>> = edges
        .iter()
        .filter_map(|e| e.map(start))
        .flat_map(|n| {
            if !is_lower(n) || !visited.contains(&n) {
                // Upper or new
                let mut v = Vec::with_capacity(visited.len() + 1);
                v.extend_from_slice(&visited);
                paths2(edges, n, end, v, revisited)
            } else if !revisited && n != "start" && n != "end" {
                let mut v = Vec::with_capacity(visited.len() + 1);
                v.extend_from_slice(&visited);
                paths2(edges, n, end, v, true)
            } else {
                Vec::new()
            }
        })
        .collect();

    sub_paths
}

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
