use crate::utils::{file, solution::{SolutionPair, Solution}};

pub fn part1(file_string: String) -> i32 {
    let lines = file_string.split_terminator("\n");
    let mut sum: i32 = 0;

    for line in lines {
        let numbers = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = numbers[0].split_terminator(" ").filter_map(|number| number.parse::<u32>().ok()).collect::<Vec<_>>();
        let my_numbers = numbers[1].split_terminator(" ").filter_map(|number| number.parse::<u32>().ok()).collect::<Vec<_>>();
        let mut current_count: u32 = 0;
        
        for my_number in my_numbers.iter() {
            if winning_numbers.contains(my_number) {
                current_count += 1;
            }
        }

        sum += 2_i32.pow(current_count - 1_u32);
    }
    sum 

}

pub fn part2(file_string: String) -> i32 {
    let lines = file_string.split_terminator("\n");
    let mut cards_per_game: Vec<i32>  = vec![1; lines.clone().count()]; 

    for (i,line) in lines.enumerate() {
        for _ in 0..cards_per_game[i]  {
            let numbers = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();
            let winning_numbers = numbers[0].split_terminator(" ").filter_map(|number| number.parse::<u32>().ok()).collect::<Vec<_>>();
            let my_numbers = numbers[1].split_terminator(" ").filter_map(|number| number.parse::<u32>().ok()).collect::<Vec<_>>();

            let mut current_count: usize = 0;
            
            for my_number in my_numbers.iter() {
                if winning_numbers.contains(my_number) {
                    cards_per_game[i+current_count+1] += 1;
                    current_count += 1;
                }
            }
        }
    }
   cards_per_game.into_iter().reduce(|acc, card_number| acc + card_number).unwrap()
    
}

pub fn solve(input_path: &str) -> SolutionPair {
    let file_string = file::read_file(input_path);
    let sol1 = part1(file_string.clone());
    let sol2 = part2(file_string.clone());

    (Solution::from(sol1), Solution::from(sol2))
}
