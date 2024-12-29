use crate::{utils, Solution};

pub fn solve() -> Solution {
    let data = parse_input();
    let mut solution = Solution::new("day 010");
    solution
}

fn parse_input() -> Vec<String> {
    utils::get_data_string(010, utils::DataSource::Test)
        .lines()
        .map(|line| line.to_string())
        .collect()
}
