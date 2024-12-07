use std::{collections::HashMap, hash::Hash};
use crate::utils::{get_data_string, DataSource, Part, Solution};

#[derive(Debug)]
enum Direction  {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Guard {
    x: usize,
    y: usize,
    direction: Direction,
    visited: HashMap<(usize, usize), usize>,
}

impl Guard {
    fn new (x: usize, y: usize) -> Guard {
        let mut visited = HashMap::new();
        visited.insert((x,y), 1);
        Guard {
            x,
            y,
            direction: Direction::Up,
            visited,
        }
    }
    fn walk_forward (&mut self) {
        match self.direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
        self.visited.entry((self.x, self.y)).or_insert(1);
    }
    fn rotate_right (&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn parse_input () -> Vec<Vec<char>> {
    get_data_string(6,DataSource::Data)
        .lines()
        .map(|x| 
            x.chars().collect::<Vec<char>>()
        ).collect()
}

fn find_and_create_guard (data: &Vec<Vec<char>>) -> Option<Guard> {
    let (height, width) = (data.len(), data[0].len());
    let mut guard: Option<Guard> = None;

    for (y_pos, y) in data.iter().enumerate() {
        if y.contains(&'^') {
            let guard_x = y.iter().position(|&c| c == '^').unwrap();
            guard = Some(Guard::new(guard_x, y_pos));
            break;
        }
    }
    guard
}

fn get_char_in_front_of_guard(guard: &Guard, data: &Vec<Vec<char>>) -> Option<char> {
    let (x, y) = match guard.direction {
        Direction::Up if guard.y > 0 => (guard.x, guard.y - 1),
        Direction::Down if guard.y < data.len() - 1 => (guard.x, guard.y + 1),
        Direction::Left if guard.x > 0 => (guard.x - 1, guard.y),
        Direction::Right if guard.x < data[0].len() - 1 => (guard.x + 1, guard.y),
        _ => return None,
    };

    if pos_in_bounds(x, y, data) {
        Some(data[y][x])
    } else {
        None
    }
}

fn pos_in_bounds (x: usize, y: usize, data: &Vec<Vec<char>>) -> bool {
    let (height, width) = (data.len(), data[0].len());
    x < width && y < height
}

fn guard_in_bounds (guard: &Guard, data: &Vec<Vec<char>>) -> bool {
    pos_in_bounds(guard.x, guard.y, data)
}

pub fn solve () -> Solution {

    let mut solution: Solution = Solution::new("day 6");
    let data: Vec<Vec<char>> = parse_input();
    let mut guard: Guard = find_and_create_guard(&data).unwrap();

    while guard_in_bounds(&guard, &data) {
        let char_in_front_of_guard = get_char_in_front_of_guard(&guard, &data);
        match char_in_front_of_guard {
            Some('#') => {
                guard.rotate_right();
                guard.walk_forward();
            },
            Some('.') | Some('^')=> {
                guard.walk_forward();
            },       
            None => {
                break;
            },
            _ => {
                panic!("Unexpected character found at position ({}, {})", guard.x, guard.y);
            }
        }
    }

    let unique_locations_visited = &guard.visited.iter().count();
    solution.increment(Part::One, *unique_locations_visited);
    solution
}

