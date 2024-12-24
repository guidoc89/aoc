use crate::utils::file;
use crate::utils::solution::{Solution, SolutionPair};

fn is_safe(line: &Vec<i32>) -> bool {
    let mut safe = false;
    let is_ascending = line.windows(2).all(|w| w[0] < w[1]);
    let is_descending = line.windows(2).all(|w| w[0] > w[1]);
    let is_valid_diff = line.windows(2).all(|w| {
        let diff = (w[0] - w[1]).abs();
        (1..=3).contains(&diff)
    });

    (is_ascending || is_descending) && is_valid_diff
}
fn part1(file_string: String) -> usize {
    file_string
        .split_terminator("\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| is_safe(&line))
        .count()
}

fn part2(file_string: String) -> usize {
    file_string
        .split_terminator("\n")
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|line| {
            let mut safe = is_safe(&line);
            if !safe {
                for i in 0..line.len() {
                    // NOTE: this would be a way to slice it like in python? line[i:] + line[i+1:].
                    let mut new_line_2 = line.iter().enumerate().filter(|&(idx, val)| idx != i).map(|(_, val)| *val).collect::<Vec<_>>();
                    // let mut new_line = line.clone();
                    // new_line.remove(i);
                    // if is_safe(&new_line) {
                    if is_safe(&new_line_2) {
                        safe = true;
                        break
                    }
                }
            }
            safe
        })
        .count()
}

pub fn solve(file_path: &str) -> SolutionPair {
    let file_string = file::read_file(file_path);

    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
