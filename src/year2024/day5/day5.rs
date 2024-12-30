use std::collections::{HashMap, VecDeque};

use crate::utils::file;
use crate::utils::solution::{Solution, SolutionPair};


fn part1(file_string: String) -> i32 {
    let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut ans = 0;
    let blocks: Vec<_> = file_string
        .split_terminator("\n\n")
        .map(|block| block.split_terminator("\n").collect::<Vec<_>>())
        .collect();
    let pages = &blocks[0]
        .iter()
        .map(|line| {
            let nums = line.split("|")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let l = nums[0];
            let r = nums[1];
            hm.entry(l).or_insert(Vec::new()).push(r);
            nums
        })
        .collect::<Vec<_>>();
    let updates = &blocks[1]
        .iter()
        .map(|line| {
            let mut correct = true;
            let nums = line.split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            nums
        })
        .collect::<Vec<_>>();

    for update in updates {
        let mut correct = true;
        for (i, x) in update.iter().enumerate() {
            for (j, y) in update.iter().enumerate() {
                if i < j {
                    if let Some(nums) = hm.get(&x) {
                        if !nums.contains(y) {
                            correct = false;
                            break;
                        }
                    } else {
                        correct = false;
                        break;
                    }
                }
            }
        }

        if correct {
            ans += update[update.len() / 2];
        }

    }

    ans
}

fn part2(file_string: String) -> i32 {
    let mut hm: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut ans = 0;
    let blocks: Vec<_> = file_string
        .split_terminator("\n\n")
        .map(|block| block.split_terminator("\n").collect::<Vec<_>>())
        .collect();
    let pages = &blocks[0]
        .iter()
        .map(|line| {
            let nums = line.split("|")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            let l = nums[0];
            let r = nums[1];
            // dont need to defer hm, push instead of append
            hm.entry(l).or_insert(Vec::new()).push(r);
            nums
        })
        .collect::<Vec<_>>();
    let updates = &blocks[1]
        .iter()
        .map(|line| {
            let mut correct = true;
            let nums = line.split(",")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>();
            nums
        })
        .collect::<Vec<_>>();

    // TODO: topological sort
    for update in updates {
        let mut correct = true;
        for (i, x) in update.iter().enumerate() {
            for (j, y) in update.iter().enumerate() {
                if i < j {
                    if let Some(nums) = hm.get(&x) {
                        if !nums.contains(y) {
                            correct = false;
                            break;
                        }
                    } else {
                        correct = false;
                        break;
                    }
                }
            }
        }

        if !correct {
            let mut topo_order: Vec<i32> = Vec::new();
            // let mut in_degree = update.iter().enumerate().map(|(idx, u)| (*u, 0)).collect::<HashMap<i32,i32>>();
            let mut in_degree = update.iter().map(|&u| (u, 0)).collect::<HashMap<i32,i32>>();
            let mut q: VecDeque<i32> = VecDeque::new();
            
            // Update in_degree, depending on whether they have outgoing nodes or not, otherwise it
            // means that they are the first ones
            for node in update {
                if let Some(neighbours) = hm.get(&node) {
                    for &neighbour in neighbours {
                        if update.contains(&neighbour) {
                            *in_degree.entry(neighbour).or_insert(0) += 1
                        }
                    }
                }
            }

            // Add the ones that are first to the q
            for (&node, &degree) in &in_degree {
                if degree == 0 {
                    q.push_back(node);
                }
            }

            while let Some(node) = q.pop_front() {
                topo_order.push(node);

                if let Some(neighbours) = hm.get(&node) {
                    for &neighbour in neighbours {
                        if let Some(degree) = in_degree.get_mut(&neighbour) {
                            *degree -= 1;
                            if *degree == 0 {
                                q.push_back(neighbour);
                            }
                        }
                    }
                }
            }

            ans += topo_order[topo_order.len() / 2];
        }
    }

    ans
}


pub fn solve(file_path: &str) -> SolutionPair {
    let file_string = file::read_file(file_path);

    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
