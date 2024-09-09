use std::collections::HashMap;
use crate::utils::{file, solution::{Solution, SolutionPair}};


fn part1(file_string: String) -> usize{
    let file_lines: Vec<&str> = file_string.split_terminator("\n\n").collect();
    let directions = file_lines[0];
    let full_map = file_lines[1..].into_iter().flat_map(|line| line.split_terminator("\n").collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut ans: usize = 0;

    let mut left_map: HashMap<&str, &str> = HashMap::new();
    let mut right_map: HashMap<&str, &str> = HashMap::new();

    let mut start_node =  "AAA";
    let end_node =  "ZZZ";

    for network in full_map {
        let root = network.get(0..3).unwrap();
        let left = network.get(7..10).unwrap();
        let right = network.get(12..15).unwrap();
        left_map.insert(root, left);
        right_map.insert(root, right);
    };

    while start_node != end_node {
        for direction in directions.chars() {
            if direction == 'L' {
                start_node = left_map.get(start_node).unwrap();
                ans += 1
            } else if direction == 'R' {
                start_node = right_map.get(start_node).unwrap();
                ans += 1
            } else {
                panic!("another direction")
            }
        }
    }

    ans
}


fn gcd(a: usize, b: usize) -> usize {
    if b == 0 { a } else { gcd(b, a % b) }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}


fn calculate_steps(node: &str, left_map: &HashMap<&str, &str>, right_map: &HashMap<&str, &str>, directions: &str) -> usize {
    let mut steps = 0;
    let mut curr_node = node;

    while !curr_node.ends_with('Z') {
        for direction in directions.chars() {
            if direction == 'L' {
                curr_node = left_map.get(curr_node).unwrap_or(&curr_node);
            } else {
                curr_node = right_map.get(curr_node).unwrap_or(&curr_node);
            }
            steps += 1
        }
    }

    steps
}


fn part2(file_string: String) -> usize {
    let file_lines: Vec<&str> = file_string.split_terminator("\n\n").collect();
    let directions = file_lines[0];
    let full_map = file_lines[1..].into_iter().flat_map(|line| line.split_terminator("\n").collect::<Vec<_>>()).collect::<Vec<_>>();
    let mut ans: usize = 0;

    let mut left_map: HashMap<&str, &str> = HashMap::new();
    let mut right_map: HashMap<&str, &str> = HashMap::new();


    let mut starting_nodes: Vec<&str> = Vec::new();

    for network in full_map {
        let root = network.get(0..3).unwrap();
        let left = network.get(7..10).unwrap();
        let right = network.get(12..15).unwrap();
        left_map.insert(root, left);
        right_map.insert(root, right);

        if root.ends_with("A") { starting_nodes.push(root) };
    };

    // Calculate the LCM of all the steps required for each starting node, update as we check how
    // many steps it takes for each node that starts with 'A'
    let mut result = 1;
    for node in starting_nodes {
        // NOTE: theoretically this approach is wrong, but this particular input has been
        // constructed in a way that this works, but its not a generalizable approach
        let steps = calculate_steps(node, &left_map, &right_map, directions);
        result = lcm(result, steps);
    };

    result
}

pub fn solve(input_path: &str) -> SolutionPair{
    let file_string = file::read_file(input_path);
    // let sol1 = part1(file_string.clone());
    let sol1 = 0;
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
