use std::{collections::HashMap, fs};
use crate::utils::{file, solution::{SolutionPair, Solution}};

pub fn part1(file_string: String) -> i64 {
    let mut blocks = file_string.split_terminator("\n\n");

    // The input seeds are the first row
    let mut seeds: Vec<i64> = blocks
        .next()
        .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for (i, block) in blocks.enumerate() {
        // println!("Inside the block");
        let mut lines: Vec<&str> = block.split_terminator("\n").collect();
        let mappings: Vec<&str> = lines
            .first()
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .split("-")
            .collect::<Vec<&str>>();

        let mut source: &str = mappings.get(0).unwrap();
        let mut destination: &str = mappings.get(2).unwrap();
        let mut conversions: HashMap<i64, i64> = HashMap::new();

        for seed in seeds.iter_mut() {
            // Since we dont care about the fist one (they are the seeds)
            for line in &lines[1..] {
                // println!("line: {:?}", line);
                let numbers: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                let from = *numbers.get(1).unwrap();
                let to = *numbers.get(0).unwrap();
                let range = *numbers.get(2).unwrap();

                if *seed >= from && *seed <= (from + range - 1) {
                    *seed = to + (*seed - from);
                    break;
                } else {
                    continue;
                }
            }
        }
    }

    let min_value = seeds.iter().min().unwrap();
    *min_value
}


pub fn part2(file_string: String) {
    let mut blocks = file_string.split_terminator("\n\n").collect::<Vec<_>>();
    // let mut blocks = file_string.split_terminator("\n\n");
    // The input seeds are the first row
    let mut seeds: Vec<i64> = blocks[0]
        // .next()
        // .unwrap()
        .split(":")
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();
    let mut seed_pairs: Vec<(i64, i64)> = seeds
        .chunks(2)
        // .to_owned()
        // .collect::<Vec<_>>()
        // .iter()
        // .map(|&pair| (pair.to_vec()[0], pair.to_vec()[0] + pair.to_vec()[1]))
        .map(|pair| (pair[0], pair[0] + pair[1]))
        .collect::<Vec<_>>();
    let mut solutions: Vec<(i64, i64)> = Vec::new();

    while !seed_pairs.is_empty() {
        let (seed_start, seed_end) = seed_pairs.pop().unwrap();
        for block in &blocks[1..] {
            let mut lines: Vec<&str> = block.split_terminator("\n").collect();

            for line in &lines[1..] {
                let numbers: Vec<i64> = line
                    .split_whitespace()
                    .map(|s| s.parse::<i64>().unwrap())
                    .collect();
                let from = *numbers.get(1).unwrap();
                let to = *numbers.get(0).unwrap();
                let range = *numbers.get(2).unwrap();
                // println!("from: {:?} - to: {:?} - range: {:?}", from, to, range);
                // TODO: when did i do this, why is this part not complete????
                
            }
        }
    }
}

pub fn solve(input_path: &str) -> SolutionPair {
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    // let sol2 = part2(file_string.clone());
    let sol2 = "Finished but removed code. TODO: need to reimplement later";

    (Solution::from(sol1), Solution::from(sol2))
}
