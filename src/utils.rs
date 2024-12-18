use std::{fs::read_to_string, path::{Path, PathBuf}};

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
    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(format!("src/day{:03}/{}", day, file_name));
    read_to_string(d).expect("please run from SRC")
}

pub struct Timer {
    start: std::time::Instant,
}

impl Timer {
    pub fn new () -> Timer {
        Timer {
            start: std::time::Instant::now(),
        }
    }

    pub fn elapsed (&self) -> std::time::Duration {
        self.start.elapsed()
    }

    pub fn reset (&mut self) {
        self.start = std::time::Instant::now();
    }

    pub fn print_elapsed (&self) {
        println!("Elapsed: {:.2?}", self.elapsed());
    }
}