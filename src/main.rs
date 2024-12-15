use std::env;

use utils::Solution;

mod utils;
mod day004;
mod day005;
mod day006;
mod day007;

fn main() {

    let solutions = vec![
        ||Solution::new("day 1"),
        ||Solution::new("day 2"),
        ||Solution::new("day 3"),
        day004::solve,
        day005::solve,
        day006::solve,
        day007::solve,
    ];

    let args: Vec<String> = env::args().collect();

    
    dbg!(&args);
    if args.len() < 2 {
        solutions.iter().for_each(|s| s().print_solution());
        
    } else {
        match args[1].parse::<i32>() {
            Ok(day) => {
                if day > 0 && day < (solutions.len() as i32 + 1) {
                    solutions[day as usize - 1]().print_solution();
                } else {
                    println!("Invalid argument - must be a day number between 1 and {}", solutions.len());
                }
            },
            Err(_) => println!("Invalid argument - must be a day number")
        }
    }
}
