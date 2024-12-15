use std::collections::{HashMap, HashSet};
use crate::utils;

fn parse_input () -> Vec<Vec<char>> { 
    utils::get_data_string(6,utils::DataSource::Data)
        .lines()
        .map(|x| 
            x.chars().collect::<Vec<char>>()
        ).collect()
}

fn get_char_at_position (data: &Vec<Vec<char>>, x: usize, y: usize ) -> char {
    data[y][x]
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}

struct GuardAndState {
    xy: (usize, usize),
    direction: Direction,
    places_visited: HashSet<(usize, usize)>,
    obstacles_hit_and_directions: HashMap<((usize, usize), Direction), usize>,
    world_dimensions: (usize, usize),
    infinite_loops_found: usize,
    part: utils::Part,
}

impl GuardAndState {
    fn step (&mut self, data: &Vec<Vec<char>>) -> Option<()> {        
        match self.get_next_position() {
            Some(next) => {
                let next_char: char = get_char_at_position(data, next.0, next.1);
                match next_char {
                    '#' => {

                        if (self.part == utils::Part::Two) {
                            // check for infinite loop
                            // if guard has already hit this obstacle from this direction
                            // guard is looping

                            if self.obstacles_hit_and_directions.contains_key(&(next, self.direction)) {
                                self.infinite_loops_found += 1;
                                return None
                            }
                            self.obstacles_hit_and_directions
                                .entry((next, self.direction)).insert_entry(1);
                        }

                        Some(self.turn_right())
                    },
                    '.' | '^' => {
                        self.xy = next;
                        if self.part == utils::Part::One { self.places_visited.insert(next); }
                        Some(())
                    },
                    _ => panic!("unexpected character")
                }
            },
            None => {
                None
            }
        }
    }

    fn turn_right (&mut self) {
        self.direction = match self.direction {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }

    fn reset (&mut self, xy: (usize, usize), part: utils::Part) {
        self.xy = xy;
        self.direction = Direction::Up;
        self.places_visited.clear();
        self.obstacles_hit_and_directions.clear();
        self.part = part;
    }

    fn get_next_position (&self) -> Option<(usize, usize)> {
        let (x, y) = self.xy;
        let (width, height) = self.world_dimensions;
        match self.direction {
            Direction::Up => {
                if y == 0 { None } else { Some((x, y-1)) }
            },
            Direction::Down => {
                if y == height - 1 { None } else { Some((x, y + 1)) }
            },
            Direction::Left => {
                if x == 0 { None } else { Some((x - 1, y)) }
            },
            Direction::Right => {
                if x == width - 1 { None } else { Some((x + 1, y)) }
            }
        }
    }
}


fn get_all_positions_with_dot (data: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();
    for (y, row) in data.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '.' {
                positions.push((x, y));
            }
        }
    }
    positions
}

fn replace_char_at_position (data: &mut Vec<Vec<char>>, x: usize, y: usize, c: char) {
    data[y][x] = c;
}

pub fn solve () -> utils::Solution {
    let mut solution = utils::Solution::new("LEVEL 6");

    let mut data = parse_input();
    let ( height, width ) = (data.len(), data[0].len());
    let caret_start_position = data
        .iter()
        .enumerate()
        .find_map(|(y, row)| 
            row.iter().position(|&c| c == '^').map(|x| (x, y))
        ).unwrap();

    let mut guard = GuardAndState {
        xy: caret_start_position,
        direction: Direction::Up,
        world_dimensions: (height, width),
        places_visited: HashSet::new(),
        obstacles_hit_and_directions: HashMap::new(),
        infinite_loops_found: 0,
        part: utils::Part::One,
    };

    //
    // begin part 1
    //

    while let Some(_) = guard.step(&data) {}
    solution.increment(utils::Part::One, guard.places_visited.len());

    //
    // begin part 2
    //

    guard.reset(caret_start_position, utils::Part::Two);

    let possible_locations_for_obstructions = get_all_positions_with_dot(&data);
    possible_locations_for_obstructions.iter().for_each(|(x, y)| {

        // replace dot with obstruction
        replace_char_at_position(&mut data, *x, *y, '#');

        while let Some(_) = guard.step(&data) {}        
        // undo obstruction placement for next iteration
        replace_char_at_position(&mut data, *x, *y, '.');
        guard.reset(caret_start_position, utils::Part::Two);
    });

    solution.increment(utils::Part::Two, guard.infinite_loops_found);

    solution
}