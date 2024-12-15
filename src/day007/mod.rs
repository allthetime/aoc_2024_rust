use crate::{utils, Solution};

enum Operation {
    Add,
    Multiply,
}

fn calculate_next_number(current: usize, total: usize, test_numbers: &Vec<usize>, operation: Operation) -> usize {
    if test_numbers.len() == 1 {
        return match operation {
            Operation::Add => current + test_numbers[0],
            Operation::Multiply => current * test_numbers[0],
        };
    }

    let first_number = &test_numbers[0];
    let rest = &test_numbers[1..];

    let current = match operation {
        Operation::Add => current + first_number,
        Operation::Multiply => current * first_number,
    };

    calculate_next_number(current, total, &rest.to_vec(), operation)
}

fn try_all_combinations(total: usize, test_numbers: &Vec<usize>) -> bool {
    fn helper(current: usize, total: usize, test_numbers: &[usize]) -> bool {
        if test_numbers.is_empty() {
            return current == total;
        }

        let first_number = test_numbers[0];
        let rest = &test_numbers[1..];

        helper(current + first_number, total, rest) || helper(current * first_number, total, rest)
    }

    helper(0, total, test_numbers)
}

pub fn solve() -> Solution {
    let data = parse_input();
    let mut solution = Solution::new("day 7");
    // ...existing code to solve the problem...
    data.iter().for_each(|(total, test_numbers)| {
        if try_all_combinations(*total, test_numbers) {
            println!("Possible to achieve total: {}", total);
            solution.increment(utils::Part::One, *total);
        } else {
            println!("Not possible to achieve total: {}", total);
        }
    });

    solution
}

fn parse_input() -> Vec<(usize, Vec<usize>)> {
    utils::get_data_string(7, utils::DataSource::Test)
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(':').collect();
            let first = parts[0].trim().parse().unwrap();
            let second = parts[1]
                .split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            (first, second)
        }).collect()
}
