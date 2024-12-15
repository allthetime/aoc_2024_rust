use std::{fs::read_to_string, path::Path};

pub struct Solution {
    part1: usize,
    pub part2: usize,
    label: &'static str,
}

#[derive(PartialEq)]
pub enum Part {
    One,
    Two,
}

impl Solution {
    pub fn new (label: &'static str) -> Solution {
        Solution {
            part1: 0,
            part2: 0,
            label,
        }
    }

    pub fn increment (&mut self, part: Part, value: usize) {
        match part {
            Part::One => self.part1 += value,
            Part::Two => self.part2 += value,
            _ => (),
        }
    }

    pub fn modify (&mut self, part: Part, modifier: fn(usize) -> usize) {
        match part {
            Part::One => self.part1 = modifier(self.part1),
            Part::Two => self.part2 = modifier(self.part2),
            _ => (),
        }
    }

    fn get_solution (&self) -> (usize, usize) {
        (self.part1, self.part2)
    }

    pub fn print_solution (&self) {
        let solution = self.get_solution();
        println!("{}\npart1: {} \npart2: {}", self.label, solution.0, solution.1);
    }

}

pub enum DataSource {
    Test,
    Data
}

fn get_filename (source: DataSource) -> &'static str {
    match source {
        DataSource::Test => "test.dat",
        DataSource::Data => "data",
    }
}

pub fn get_data_string (day:usize, source: DataSource) -> String {
    let file_name = get_filename(source);
    let path = format!("day{:03}/{}", day, file_name);
    read_to_string(path).expect("please run from SRC")
}
