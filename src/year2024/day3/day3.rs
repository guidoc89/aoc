use std::cell::RefCell;

use crate::utils::file;
use crate::utils::solution::{Solution, SolutionPair};

const PAT: &str = "mul(";

// NOTE: just another way
// fn part1(file_string: String) -> i32 {
//     let mut ans = 0;
//     let file_string = file_string.replace("\n", "");
//     for (idx,c) in file_string.chars().enumerate() {
//         let mut j = idx;
//         if file_string[idx..].starts_with(PAT) {
//             while file_string.chars().nth(j).unwrap() != ')' {
//                 j += 1
//             }
//
//             let possible = &file_string[idx..=j].replace(PAT, "").replace(")", "");
//             let splits: Vec<_> = possible.split(",").collect();
//
//                 if splits.len() == 2 {
//                     if let (Ok(num1), Ok(num2)) = (splits[0].trim().parse::<i32>(), splits[1].trim().parse::<i32>()) {
//                         ans += (num1 * num2);
//                     }
//                 }
//         }
//     }
//     ans
// }

fn part1(file_string: String) -> i32 {
    file_string
        .replace("\n", "")
        .chars()
        .enumerate()
        .filter_map(|(idx, _)| {
            if file_string[idx..].starts_with(PAT) {
                let j = file_string[idx..]
                    .chars()
                    .position(|c| c == ')')
                    .map(|pos| pos + idx)?;

                let possible = &file_string[idx..=j].replace(PAT, "").replace(")", "");
                let splits: Vec<&str> = possible.split(",").collect();
                if splits.len() == 2 {
                    if let (Ok(n1), Ok(n2)) = (
                        splits[0].trim().parse::<i32>(),
                        splits[1].trim().parse::<i32>(),
                    ) {
                        return Some(n1 * n2);
                    }
                }
            }
            None
        })
        .sum()
}

// NOTE: this works, but find another way to solve it without the mutable RefCell inside the filter_map closure, drop the filter_map
// and handle it differently
fn part2(file_string: String) -> i32 {
    let enabled = RefCell::new(true);

    file_string
        .replace("\n", "")
        .chars()
        .enumerate()
        .filter_map(|(idx, _)| {
            if file_string[idx..].starts_with("do()") {
                *enabled.borrow_mut() = true;
                return None;
            }
            if file_string[idx..].starts_with("don't()") {
                *enabled.borrow_mut() = false;
                return None;
            }

            if file_string[idx..].starts_with(PAT) {
                let j = file_string[idx..]
                    .chars()
                    .position(|c| c == ')')
                    .map(|pos| pos + idx)?;

                let possible = &file_string[idx..=j].replace(PAT, "").replace(")", "");
                let splits: Vec<&str> = possible.split(",").collect();
                if *enabled.borrow() {
                    if splits.len() == 2 {
                        if let (Ok(n1), Ok(n2)) = (
                            splits[0].trim().parse::<i32>(),
                            splits[1].trim().parse::<i32>(),
                        ) {
                            return Some(n1 * n2);
                        }
                    }
                }
            }
            None
        })
        .sum()
}

pub fn solve(file_path: &str) -> SolutionPair {
    let file_string = file::read_file(file_path);

    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
