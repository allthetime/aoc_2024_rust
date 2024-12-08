use std::{collections::HashMap, hash::Hash};
use crate::utils::{get_data_string, DataSource, Part, Solution};

pub(crate) mod other_guy_solution;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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
    visited: HashMap<((usize, usize), Direction), usize>,
}

impl Guard {
    fn new (x: usize, y: usize) -> Guard {
        let mut visited = HashMap::new();
        visited.insert(((x,y), Direction::Up), 1);
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
        self.visited.entry(((self.x, self.y), self.direction)).and_modify(|v| *v += 1).or_insert(1);
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

fn get_char_in_front_of_guard(guard: &Guard, data: &Vec<Vec<char>>) -> Option<(char, (usize,usize))> {
    let (x, y) = match guard.direction {
        Direction::Up if guard.y > 0 => (guard.x, guard.y - 1),
        Direction::Down if guard.y < data.len() - 1 => (guard.x, guard.y + 1),
        Direction::Left if guard.x > 0 => (guard.x - 1, guard.y),
        Direction::Right if guard.x < data[0].len() - 1 => (guard.x + 1, guard.y),
        _ => return None,
    };

    if pos_in_bounds(x, y, data) {
        Some((data[y][x], (x,y)))
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

fn data_clone_with_obstacle_added_at_position (data: &Vec<Vec<char>>, x: usize, y: usize) -> Vec<Vec<char>> {
    let mut new_data = data.clone();
    new_data[y][x] = '0';
    new_data
}

fn simulate_walk (guard: & mut Guard, data: &Vec<Vec<char>>, solution: &mut Solution) -> () {

    while guard_in_bounds(&guard, data) {

        let (char_in_front_of_guard, pos_of_char) = match get_char_in_front_of_guard(&guard, data) {
            Some((c, pos)) => (Some(c), pos),
            None => (None, (0,0)),
        };

        match char_in_front_of_guard {
            Some('#') | Some('0') => {

                guard.rotate_right();
                guard.walk_forward();

                match guard.visited.get(&((guard.x, guard.y), guard.direction)) {
                    Some(v) => {
                        if *v > 1 {
                            solution.increment(Part::Two, 1);
                            break;
                        }
                    },
                    None => {

                    }
                }

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
}

fn iterate_through_all_possible_positions (original_guard: &Guard, data: &Vec<Vec<char>>, solution: &mut Solution, original_guard_position: (usize, usize)) -> () {
    let visited_positions = filter_out_visited_with_duplicate_positions(original_guard);
    for (x,y) in visited_positions.into_iter().filter(|(x,y)| (*x, *y) != original_guard_position)
        .filter(|(x,y)| data[*y][*x] == '.') {
            let new_data = data_clone_with_obstacle_added_at_position(data, x, y);
            let mut guard = find_and_create_guard(&data).unwrap();
            simulate_walk(&mut guard, &new_data, solution);
        }
}

fn filter_out_visited_with_duplicate_positions (guard: &Guard) -> Vec<((usize, usize))> {
    let mut visited_positions = guard.visited.keys().map(|((x,y), direction)| ((*x, *y))).collect::<Vec<((usize, usize))>>();
    visited_positions.sort();
    visited_positions.dedup();
    visited_positions
}

pub fn solve () -> Solution {

    let mut solution: Solution = Solution::new("day 7");
    let data: Vec<Vec<char>> = parse_input();
    let mut guard: Guard = find_and_create_guard(&data).unwrap();

    let original_guard_position = (guard.x, guard.y);

    simulate_walk(&mut guard, &data, &mut solution);
    let unique_locations_visited = filter_out_visited_with_duplicate_positions(&guard).len();
    solution.increment(Part::One, unique_locations_visited);

    iterate_through_all_possible_positions(&guard, &data, &mut solution, original_guard_position);

    solution
}

