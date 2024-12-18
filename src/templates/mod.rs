use crate::{utils, Solution};

pub fn solve() -> Solution {
    let data = parse_input();
    let mut solution = Solution::new("day {DAY_NUMBER}");
    solution
}

fn parse_input() -> Vec<String> {
    utils::get_data_string({ DAY_NUMBER }, utils::DataSource::Test)
        .lines()
        .map(|line| line.to_string())
        .collect()
}
