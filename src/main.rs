use utils::{cli::parse_arguments, year::match_year_and_day};

mod utils;
mod year2022;
mod year2023;
mod year2024;

fn main() {
    match parse_arguments() {
        Ok((year, day, file_input)) => {
            let (sol1, sol2) = match_year_and_day(&year, &day, file_input);
            println!("Sol1: {} - Sol2: {}", sol1, sol2);
        }
        Err(e) => {
            eprintln!("{}", e)
        }
    }
}
