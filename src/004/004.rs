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

const iterable_directions: [Directions; 8] = [
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

fn find_adjacent_chars_in_data(char_to_find: char, data: &Vec<char>, cursor: usize, width: usize, height: usize, force_direction: Option<Directions>) -> Result< Vec<(usize,Directions)>, &'static str> {
    let mut adjacent_chars = Vec::new();
    for direction in iterable_directions.iter().filter( |d| match force_direction { 
        Some(force_direction) => **d == force_direction,
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

    let mut total_xmas = 0;

    while cursor < area {

        current_char = chars_without_newlines[cursor];
        match current_char {
            'X' => {
                let adjacent_Ms = find_adjacent_chars_in_data('M', &chars_without_newlines, cursor, width, height, None);
                for (adjacent_M, direction) in adjacent_Ms.unwrap() {
                    let adjacent_As = find_adjacent_chars_in_data('A', &chars_without_newlines, adjacent_M, width, height, Some(direction));
                    for (adjacent_A, _) in adjacent_As.unwrap() {
                        let adjacent_Ss = find_adjacent_chars_in_data('S', &chars_without_newlines, adjacent_A, width, height, Some(direction)).unwrap();
                        total_xmas += adjacent_Ss.len();
                    }
                }
            },
            _ => {
                ()
            },
        }
        cursor += 1;
    }

    println!("Total Xmas: {}", total_xmas);

}
