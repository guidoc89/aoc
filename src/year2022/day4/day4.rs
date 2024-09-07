use std::fs;

pub fn part1(file_path: &str) {
    let file_string: String = fs::read_to_string(file_path).unwrap();
    let lines = file_string.split_terminator("\n");
    let mut sum: i32 = 0;

    for line in lines {
        let numbers = line.split(":").collect::<Vec<&str>>()[1].split("|").collect::<Vec<&str>>();
        let winning_numbers = numbers[0].split_terminator(" ").filter_map(|number| number.parse::<u32>().ok()).collect::<Vec<_>>();
        let my_numbers = numbers[1].split_terminator(" ").filter_map(|number| number.parse::<u32>().ok()).collect::<Vec<_>>();

        let mut current_count: u32 = 0;
        //
        for my_number in my_numbers.iter() {
            if winning_numbers.contains(my_number) {
                current_count += 1;
            }
        }

        sum += 2_i32.pow(current_count - 1_u32);
    }
        println!("{:?}", sum);

}

pub fn part2(file_path: &str) {
    let file_string: String = fs::read_to_string(file_path).unwrap();
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
        println!("{:?}", cards_per_game.into_iter().reduce(|acc, card_number| acc + card_number).unwrap());
}


pub fn solve(file_path: &str) {
    println!("Solution for 2022 day 1 - Input mode: {:?}", file_path);
}

