use std::collections::{HashMap, HashSet, VecDeque};
use std::ops::RangeBounds;

use crate::utils::file;
use crate::utils::solution::{Solution, SolutionPair};

fn is_line_correct(nums: &Vec<usize>, idx: usize, curr_val: usize, target: usize) -> bool {
    if idx == nums.len() {
        return curr_val == target;
    }

    let next_num = nums.get(idx).unwrap();

    let mut add_res = is_line_correct(nums, idx+1, curr_val+ next_num, target);
    let mut mul_res = is_line_correct(nums, idx+1, curr_val* next_num, target);

    add_res || mul_res
}

fn is_line_correct_part_2(nums: &Vec<usize>, idx: usize, curr_val: usize, target: usize) -> bool {
    if idx == nums.len() {
        return curr_val == target;
    }

    let next_num = nums.get(idx).unwrap();
    let concat_num = format!("{}{}", curr_val, next_num).parse::<usize>().unwrap();

    let mut add_res = is_line_correct_part_2(nums, idx+1, curr_val+ next_num, target);
    let mut mul_res = is_line_correct_part_2(nums, idx+1, curr_val* next_num, target);
    let mut concat_res = is_line_correct_part_2(nums, idx+1, concat_num, target);

    add_res || mul_res || concat_res
}


fn part1(file_string: String) -> usize {
    let mut ans: usize = 0;
    let target_nums: Vec<(usize, Vec<usize>)> = file_string
        .lines()
        .map(|line| {
            let comb = line.split(':').collect::<Vec<&str>>();
            let target = comb[0].parse::<usize>().unwrap();
            let nums = comb[1]
                .split_ascii_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .collect();

    for (target, nums) in target_nums {
        if is_line_correct(&nums, 0, 0, target)  {
            ans += target;
        }
    }
    ans
}

fn part2(file_string: String) -> usize {
    let mut ans: usize = 0;
    let target_nums: Vec<(usize, Vec<usize>)> = file_string
        .lines()
        .map(|line| {
            let comb = line.split(':').collect::<Vec<&str>>();
            let target = comb[0].parse::<usize>().unwrap();
            let nums = comb[1]
                .split_ascii_whitespace()
                .map(|c| c.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            (target, nums)
        })
        .collect();

    for (target, nums) in target_nums {
        if is_line_correct_part_2(&nums, 0, 0, target)  {
            ans += target;
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
