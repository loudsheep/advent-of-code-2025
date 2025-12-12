use advent_of_code_2025::read_lines;
use std::collections::HashMap;

// cout paths from start to target, that have visited 'dac' and 'fft' nodes
fn count_paths(
    graph: &HashMap<String, Vec<String>>,
    current: &String,
    target: &String,
    memo: &mut HashMap<String, usize>,
) -> usize {
    if current == target {
        return 1;
    }

    if let Some(&cached) = memo.get(current) {
        return cached;
    }

    let mut total_paths = 0;
    if let Some(neighbors) = graph.get(current) {
        for neighbor in neighbors {
            total_paths += count_paths(graph, neighbor, target, memo);
        }
    }

    memo.insert(current.clone(), total_paths);
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
        let edges: Vec<String> = parts[1].split(" ").map(|s| s.to_string()).collect();
        graph.insert(node, edges);
    }

    // scenario 1: svr -> dac -> fft -> out
    let mut memo = HashMap::new();
    let svr_to_dac = count_paths(&graph, &"svr".to_string(), &"dac".to_string(), &mut memo);

    let mut memo = HashMap::new();
    let dac_to_fft = count_paths(&graph, &"dac".to_string(), &"fft".to_string(), &mut memo);

    let mut memo = HashMap::new();
    let fft_to_out = count_paths(&graph, &"fft".to_string(), &"out".to_string(), &mut memo);

    let total_paths_scenario_1 = svr_to_dac * dac_to_fft * fft_to_out;

    // scenario 2: svr -> fft -> dac -> out
    let mut memo = HashMap::new();
    let svr_to_fft = count_paths(&graph, &"svr".to_string(), &"fft".to_string(), &mut memo);

    let mut memo = HashMap::new();
    let fft_to_dac = count_paths(&graph, &"fft".to_string(), &"dac".to_string(), &mut memo);

    let mut memo = HashMap::new();
    let dac_to_out = count_paths(&graph, &"dac".to_string(), &"out".to_string(), &mut memo);

    let total_paths_scenario_2 = svr_to_fft * fft_to_dac * dac_to_out;

    let total_paths = total_paths_scenario_1 + total_paths_scenario_2;
    println!("Total paths from svr to out via dac and fft: {}", total_paths);
}
