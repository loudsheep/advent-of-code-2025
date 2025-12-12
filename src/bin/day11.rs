
use std::collections::HashMap;

use advent_of_code_2025::read_lines;

fn count_paths(graph: &HashMap<String, Vec<String>>, current: &String, target: &String, path_count: u32) -> u32 {
    if current == target {
        return path_count + 1;
    }

    let mut total_paths = path_count;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths(graph, neighbor, target, 0);
        }
    }

    total_paths
}

fn main() {
    let lines: Vec<String> = read_lines("input/day11.txt").expect("Could not read input file");

    // line format [startNode]: [edge1 edge2 ...]
    // aaa: you hhh
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for line in lines {
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0].to_string();
        let edges: Vec<String> = parts[1]
            .split(" ")
            .map(|s| s.to_string())
            .collect();
        graph.insert(node, edges);
    }

    let start_node = "you".to_string();
    let target_node = "out".to_string();
    let total_paths = count_paths(&graph, &start_node, &target_node, 0);
    println!("Total paths from '{}' to '{}': {}", start_node, target_node, total_paths);
}