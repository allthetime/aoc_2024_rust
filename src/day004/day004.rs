use std::fs::read_to_string;

const FILENAME: &str = "data";

fn parse_input () -> String {
    read_to_string(FILENAME)
        .unwrap()
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum Directions {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

const ITERABLE_DIRECTIONS: [Directions; 8] = [
    Directions::Up,
    Directions::UpRight, 
    Directions::Right, 
    Directions::DownRight,
    Directions::Down, 
    Directions::DownLeft,
    Directions::Left,
    Directions::UpLeft,
];

fn bounds_check(cursor: i32, outside_bounds: bool) -> Result<usize, &'static str> {
    if outside_bounds {
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

enum Rotation {
    Opposite_180,
    Clockwise_90,
    CounterClockwise_90
}

fn rotate_direction (direction: Directions, rotation: Rotation) -> Directions {
    let direction_index = ITERABLE_DIRECTIONS.iter().position(|&d| d == direction).unwrap();
    let new_direction_index = (direction_index + match rotation {
        Rotation::Opposite_180 => 4,
        Rotation::Clockwise_90 => 2,
        Rotation::CounterClockwise_90 => 6,
    }) % 8;
    return ITERABLE_DIRECTIONS[new_direction_index];
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
    let chars = data.chars().filter(|c| c != &'\n').collect::<Vec<char>>();

    let mut cursor = 0;
    let mut current_char;

    let mut total_xmas_1 = 0;
    let mut total_xmas_2 = 0;

    while cursor < area {

        current_char = chars[cursor];

        match current_char {
            'X' => {
                let adjacent_ms = find_adjacent_chars_in_data('M', &chars, cursor, width, height, None);
                for (adjacent_m, direction) in adjacent_ms.unwrap() {
                    let adjacent_as = find_adjacent_chars_in_data('A', &chars, adjacent_m, width, height, Some(vec!(direction)));
                    for (adjacent_a, _) in adjacent_as.unwrap() {
                        let adjacent_ss = find_adjacent_chars_in_data('S', &chars, adjacent_a, width, height, Some(vec!(direction))).unwrap();
                        total_xmas_1 += adjacent_ss.len();
                    }
                }
            },
            'A' => {
                let diagonally_adjacent_ss = find_adjacent_chars_in_data('S', &chars, cursor, width, height, Some(vec!(Directions::UpLeft, Directions::UpRight, Directions::DownLeft, Directions::DownRight)));
                for (_diagonally_adjacent_s, direction) in diagonally_adjacent_ss.unwrap() {
                    let opposite_ms = find_adjacent_chars_in_data('M', &chars, cursor, width, height, Some(vec!(rotate_direction(direction, Rotation::Opposite_180))));
                    for (_opposite_m, direction1) in opposite_ms.unwrap() {
                        let rotate_adjacent_ms = find_adjacent_chars_in_data('M', &chars, cursor, width, height, Some(vec!(rotate_direction(direction1, Rotation::Clockwise_90), rotate_direction(direction1, Rotation::CounterClockwise_90))));
                        for (_adjacent_m, direction2) in rotate_adjacent_ms.unwrap() {
                            let opposite_s = find_adjacent_chars_in_data('S', &chars, cursor, width, height, Some(vec!(rotate_direction(direction2, Rotation::Opposite_180))));
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
