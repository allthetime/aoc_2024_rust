use std::fs::read_to_string;

const FILENAME: &str = "data";

fn parse_input () -> String {
    read_to_string(FILENAME)
        .unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Directions {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight
}

const ITERABLE_DIRECTIONS: [Directions; 8] = [
    Directions::Up,
    Directions::Down,
    Directions::Left,
    Directions::Right,
    Directions::UpLeft,
    Directions::UpRight,
    Directions::DownLeft,
    Directions::DownRight
];

fn bounds_check(cursor: i32, condition: bool) -> Result<usize, &'static str> {
    if condition {
        Err("Out of bounds")
    } else {
        Ok(cursor as usize)
    }
}

fn get_char_in_direction(direction: Directions, cursor: usize, width: usize, height: usize) -> Result<usize, &'static str> {
    
    let (cursor_i32, width_i32, height_i32) = (cursor as i32, width as i32, height as i32);
    
    match direction {
        Directions::Up => bounds_check(cursor_i32 - width_i32, cursor_i32 < width_i32),
        Directions::Down => bounds_check(cursor_i32 + width_i32, cursor_i32 >= width_i32 * (height_i32 - 1)),
        Directions::Left => bounds_check(cursor_i32 - 1, cursor_i32 % width_i32 == 0),
        Directions::Right => bounds_check(cursor_i32 + 1, cursor_i32 % width_i32 == width_i32 - 1),
        Directions::UpLeft => bounds_check(cursor_i32 - width_i32 - 1, cursor_i32 < width_i32 || cursor_i32 % width_i32 == 0),
        Directions::UpRight => bounds_check(cursor_i32 - width_i32 + 1, cursor_i32 < width_i32 || cursor_i32 % width_i32 == width_i32 - 1),
        Directions::DownLeft => bounds_check(cursor_i32 + width_i32 - 1, cursor_i32 >= width_i32 * (height_i32 - 1) || cursor_i32 % width_i32 == 0),
        Directions::DownRight => bounds_check(cursor_i32 + width_i32 + 1, cursor_i32 >= width_i32 * (height_i32 - 1) || cursor_i32 % width_i32 == width_i32 - 1),
    }
}

fn get_opposite_direction(direction: Directions) -> Directions {
    match direction {
        Directions::Up => Directions::Down,
        Directions::Down => Directions::Up,
        Directions::Left => Directions::Right,
        Directions::Right => Directions::Left,
        Directions::UpLeft => Directions::DownRight,
        Directions::UpRight => Directions::DownLeft,
        Directions::DownLeft => Directions::UpRight,
        Directions::DownRight => Directions::UpLeft,
    }
}

fn get_direction_90deg_clockwise(direction: Directions) -> Directions {
    match direction {
        Directions::Up => Directions::Right,
        Directions::Down => Directions::Left,
        Directions::Left => Directions::Up,
        Directions::Right => Directions::Down,
        Directions::UpLeft => Directions::UpRight,
        Directions::UpRight => Directions::DownRight,
        Directions::DownLeft => Directions::UpLeft,
        Directions::DownRight => Directions::DownLeft,
    }
}

fn get_direction_90deg_counter_clockwise(direction: Directions) -> Directions {
    match direction {
        Directions::Up => Directions::Left,
        Directions::Down => Directions::Right,
        Directions::Left => Directions::Down,
        Directions::Right => Directions::Up,
        Directions::UpLeft => Directions::DownLeft,
        Directions::UpRight => Directions::UpLeft,
        Directions::DownLeft => Directions::DownRight,
        Directions::DownRight => Directions::UpRight,
    }
}


fn find_adjacent_chars_in_data(char_to_find: char, data: &Vec<char>, cursor: usize, width: usize, height: usize, force_direction: Option<Vec<Directions>>) -> Result< Vec<(usize,Directions)>, &'static str> {
    let mut adjacent_chars = Vec::new();
    for direction in ITERABLE_DIRECTIONS.iter().filter( |d| match &force_direction { 
        Some(force_direction) => force_direction.contains(d),
        None => true
    }) {
        match get_char_in_direction(*direction, cursor, width, height) {
            Ok(new_cursor) => {
                let char_in_position = data[new_cursor];
                if char_in_position == char_to_find {
                    adjacent_chars.push((new_cursor, *direction));
                } else {
                    continue;
                }
            }
            Err(_) => continue,
        }
    }
    Ok(adjacent_chars)
}

fn main() {

    let data = parse_input();
    let lines = data.lines().collect::<Vec<_>>();
    let (height, width) = (lines.len(), lines[0].len());
    let area = height * width;
    let chars_without_newlines = data.chars().filter(|c| c != &'\n').collect::<Vec<char>>();

    let mut cursor = 0;
    let mut current_char;

    let mut total_xmas_1 = 0;
    let mut total_xmas_2 = 0;

    // scan through the data looking for starting points (X for part 1, A for part 2)

    while cursor < area {

        current_char = chars_without_newlines[cursor];

        match current_char {
            //
            // part 1
            //
            // find an X
            'X' => {
                // find Ms adjacent to the X in any direction
                let adjacent_ms = find_adjacent_chars_in_data('M', &chars_without_newlines, cursor, width, height, None);
                for (adjacent_m, direction) in adjacent_ms.unwrap() {
                    // find As adjacent to the M in the same direction the M was from the X
                    let adjacent_as = find_adjacent_chars_in_data('A', &chars_without_newlines, adjacent_m, width, height, Some(vec!(direction)));
                    for (adjacent_a, _) in adjacent_as.unwrap() {
                        // find Ss adjacent to the A in the same direction the A was from the M
                        let adjacent_ss = find_adjacent_chars_in_data('S', &chars_without_newlines, adjacent_a, width, height, Some(vec!(direction))).unwrap();
                        total_xmas_1 += adjacent_ss.len();
                    }
                }
            },
            //
            // part 2
            //
            // find an A
            'A' => {
                // find Ss diagonally adjacent to the A
                let diagonally_adjacent_ss = find_adjacent_chars_in_data('S', &chars_without_newlines, cursor, width, height, Some(vec!(Directions::UpLeft, Directions::UpRight, Directions::DownLeft, Directions::DownRight)));
                for (_diagonally_adjacent_s, direction) in diagonally_adjacent_ss.unwrap() {
                    // now find an M diagonally opposite to the S accross the A
                    let opposite_ms = find_adjacent_chars_in_data('M', &chars_without_newlines, cursor, width, height, Some(vec!(get_opposite_direction(direction))));
                    for (_opposite_m, direction1) in opposite_ms.unwrap() {
                        // now find another M next to the first M rotating 90degrees around the A
                        let rotate_adjacent_ms = find_adjacent_chars_in_data('M', &chars_without_newlines, cursor, width, height, Some(vec!(get_direction_90deg_clockwise(direction1), get_direction_90deg_counter_clockwise(direction1))));
                        for (_adjacent_m, direction2) in rotate_adjacent_ms.unwrap() {
                            // finally, find an S diagonally opposite to the A accross the second M
                            // if we find all this... we have X-MAS
                            let opposite_s = find_adjacent_chars_in_data('S', &chars_without_newlines, cursor, width, height, Some(vec!(get_opposite_direction(direction2))));
                            total_xmas_2 += opposite_s.unwrap().len();
                        }
                    }
                }
            },
            _ => {
                ()
            },
        }

        cursor += 1;
    }

    // divide final answer by 2 because algo double counts due to the symmetry of the X-MAS
    println!("part1: {}\npart2: {}", total_xmas_1, total_xmas_2 / 2);

}
