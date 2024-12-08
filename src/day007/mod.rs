use crate::utils::{get_data_string, DataSource, Solution};

fn parse_input () -> String {
    get_data_string(7,DataSource::Test)
}

pub fn solve () -> Solution {

    let mut solution: Solution = Solution::new("day 6");
    let data: String = parse_input();

    dbg!(data);

    solution
}

