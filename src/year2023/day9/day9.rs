use crate::utils::{file, solution::{Solution, SolutionPair}};


fn get_diff(numbers: &Vec<i32>) -> Vec<i32> {
    // From a vec of len (n), gets a vector of the differences between b-a (v[1]-v[0], and so on).
    // Naturally, returns a vec of len (n-1)
    numbers.iter().zip(&numbers[1..]).map(|(a,b)| b-a).collect::<Vec<i32>>()
}

fn part1(file_string: String) -> i32 {
    // solution: 1969958987
    let mut ans = 0;
    let file_lines = file_string.split_terminator("\n").collect::<Vec<_>>();

    for line in file_lines {
        let mut last_nums: Vec<i32> = Vec::new();
        let numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut diffs = numbers.clone();

        while !diffs.iter().all(|n| *n == 0) {
            last_nums.push(*diffs.last().unwrap());
            diffs = get_diff(&diffs);
        }
        ans += last_nums.iter().sum::<i32>();
    }
    
    ans
}


fn part2(file_string: String) -> i32 {
    // solution: 1068
    let mut ans = 0;
    let file_lines = file_string.split_terminator("\n").collect::<Vec<_>>();

    for line in file_lines {
        let numbers = line.split_whitespace().map(|n| n.parse::<i32>().unwrap()).collect::<Vec<i32>>();
        let mut diffs = numbers.clone();
        let mut first_nums: Vec<i32> =  Vec::new();

        while !diffs.iter().all(|n| *n == 0) {
            first_nums.push(*diffs.first().unwrap());
            diffs = get_diff(&diffs);
        }
        first_nums.reverse();
        let past_value = first_nums.clone().into_iter().reduce(|acc, e|  e - acc).unwrap();
        ans += past_value
    }
    
    ans
}

pub fn solve(input_path: &str) -> SolutionPair {
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
