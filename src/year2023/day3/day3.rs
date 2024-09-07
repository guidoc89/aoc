use crate::utils::{solution::{Solution, SolutionPair}, file};

use std::{fs, collections::{HashMap, HashSet}};

#[derive(Default, Debug)]
struct Coord {
    xi: usize,
    xf: usize,
    yi: usize,
    yf: usize
}

impl Coord {
    fn should_add(&self, other: &Coord) -> bool {
        self.xi <= other.xf && self.xf >= other.xi &&
        self.yi <= other.yf && self.yf >= other.yi
    }
}

// --------------
// ---....-------
// -------*------
// --------------


fn part1(file_string: String) {
    // let lines: Vec<&str> = file_string.split_terminator("\n").enumerate().collect();
    let lines: Vec<(usize, &str)> = file_string.split_terminator("\n").enumerate().collect();
    let mut sum: u32 = 0;
    let mut numbers_map: HashMap<u32, Coord> = HashMap::new();
    let mut symbols_coords: Vec<Coord> = Vec::new();
    let mut symbols: Vec<char> = Vec::new();


    let mut all_numbers: Vec<u32> = Vec::new();

    let mut current_number = 0;
    let mut is_number = false;

    for (y, line) in lines {

        for (x, ch) in  line.chars().enumerate() {
            let initial_offset = x - current_number.to_string().len() -1;
            let xi = if x < current_number.to_string().len() + 1  { 0 } else {initial_offset};
            let yi = if y == 0 {0} else {y-1};

            if ch != '.' && !ch.is_digit(10) {
                symbols_coords.push(Coord { xi: x, xf: x, yi: y, yf: y });
                symbols.push(ch);
                is_number = false;
                // if current_number > 0 { all_numbers.push(current_number)}
                if current_number > 0 {
                    numbers_map.insert(current_number, Coord { xi, xf: x, yi,  yf: y+1 });
                }
                current_number = 0;
            }
            if ch == '.' { 
                is_number = false;
                if current_number > 0 {
                    numbers_map.insert(current_number, Coord { xi, xf: x, yi,  yf: y+1 });
                    // all_numbers.push(current_number);
                }
                current_number = 0;
            }

            if ch.is_ascii_digit() {
                is_number = true;
                current_number = current_number*10 + ch.to_digit(10).unwrap();
                if x == line.len() - 1 {
                    is_number = false;
                    if current_number > 0 {
                        numbers_map.insert(current_number, Coord { xi, xf: x+1, yi,  yf: y+1 });
                    }
                    current_number = 0;
                }
            }

            // if is_number {
            //     current_number = current_number*10 + ch.to_digit(10).unwrap();
            //     if x == line.len() - 1 {
            //         is_number = false;
            //         if current_number > 0 {
            //             numbers_map.insert(current_number, Coord { xi, xf: x+1, yi,  yf: y+1 });
            //         }
            //         current_number = 0;
            //     }
            // }
        }
    }

    for (number, number_coord) in numbers_map.iter() {
        for symbol_coord in symbols_coords.iter() {
            if symbol_coord.should_add(number_coord) {
            // if number_coord.should_add(symbol_coord) {
                // println!("Number to add: {:?}", number);
                sum += number;
                break
            }
        }
    }

    // println!("{:?}", symbols_coords);
    // println!(" All numbers: {:?}", all_numbers);
    // println!("{:?}", numbers_map.keys());
    // println!("{:?}", numbers_map);
    // numbers_map

    // println!("{:?}", sum);
    // let set: HashSet<char> = HashSet::from_iter(symbols);
    // println!("{:?}", set);
}


pub fn solve(input_path: &str) -> SolutionPair {
    let file_string = file::read_file(input_path);
    // let sol1 = part1(file_string.clone());
    let sol1 = "Non implemented";
    // let sol2 = part2(file_string.clone());
    let sol2 =  "Non implemented";

    (Solution::from(sol1), Solution::from(sol2))
}
