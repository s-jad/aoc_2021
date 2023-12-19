use itertools::Itertools;
use std::{collections::HashMap, time::Instant};

fn dfs(
    node: &str,
    end: &str,
    visited: &mut Vec<String>,
    graph: &HashMap<&str, Vec<&str>>,
    all_paths: &mut Vec<Vec<String>>,
) {
    if node == end {
        all_paths.push(visited.clone());
        return;
    }

    if node.chars().all(|c| c.is_lowercase()) && visited.contains(&node.to_string()) {
        return;
    }

    visited.push(node.to_string());

    if let Some(neighbors) = graph.get(node) {
        for neighbor in neighbors {
            dfs(neighbor, end, visited, graph, all_paths);
        }
    }

    visited.pop();
}

fn find_all_paths(graph: HashMap<&str, Vec<&str>>, start: &str, end: &str) -> Vec<Vec<String>> {
    let mut all_paths = Vec::new();
    let mut visited = Vec::new();
    dfs(start, end, &mut visited, &graph, &mut all_paths);
    all_paths
}

fn process(input: &str) -> usize {
    let graph =
        input
            .lines()
            .map(|l| l.split(['-']).collect_vec())
            .fold(HashMap::new(), |mut acc, v| {
                acc.entry(v[0]).or_insert_with(Vec::new).push(v[1]);
                acc.entry(v[1]).or_insert_with(Vec::new).push(v[0]);
                acc
            });

    let paths = find_all_paths(graph, "start", "end");
    paths.len()
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let output = process(input);
    let time = start.elapsed();

    dbg!(output, time);
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let input = include_str!("../../test.txt");
        let output = process(input);
        assert_eq!(result,);
    }
}
