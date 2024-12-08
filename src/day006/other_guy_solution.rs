use std::collections::HashSet;

fn process_data(file_path: &str) -> Vec<Vec<char>> {
    let file_content = std::fs::read_to_string(file_path).expect("Should open a file");

    file_content
        .lines()
        .map(|line| line.chars().collect())
        .collect()
}

fn find_starting_position(map: &[Vec<char>]) -> Option<(i32, i32)> {
    for (y, row) in map.iter().enumerate() {
        if let Some(x) = row.iter().position(|&c| c == '^') {
            return Some((x as i32, y as i32));
        }
    }

    None
}

fn record_guard_movement(map: &[Vec<char>], (x, y): (i32, i32)) -> HashSet<(i32, i32)> {
    let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
    let (mut dx, mut dy): &(i32, i32) = directions.next().expect("Should produce (dx, dy)");
    let mut unique_steps: HashSet<(i32, i32)> = HashSet::from([(x, y)]);
    let (mut x, mut y): (i32, i32) = (x, y);

    loop {
        let (nx, ny) = (x + dx, y + dy);

        let cell = match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
            Some(&c) => c,
            None => return unique_steps,
        };

        match cell {
            '#' => (dx, dy) = *directions.next().expect("Should produce (dx, dy)"),
            '.' | '^' => {
                unique_steps.insert((nx, ny));
                (x, y) = (nx, ny);
            }
            c => panic!("Invalid symbol on the map: '{}'", c),
        }
    }
}

fn simulate_inserting_an_obstacle(
    map: &[Vec<char>],
    (x, y): (i32, i32),
    movement_path: &HashSet<(i32, i32)>,
) -> u64 {
    let mut infinite_loop_counter: u64 = 0;

    for obstacle in movement_path {
        let mut stop_conditions: Vec<((i32, i32), (i32, i32))> = Vec::new();
        let mut directions = [(0, -1), (1, 0), (0, 1), (-1, 0)].iter().cycle();
        let (mut dx, mut dy): &(i32, i32) = directions.next().expect("Should produce (dx, dy)");
        let (mut x, mut y): (i32, i32) = (x, y);

        loop {
            let (nx, ny) = (x + dx, y + dy);

            let mut cell = match map.get(ny as usize).and_then(|row| row.get(nx as usize)) {
                Some(&c) => c,
                None => break,
            };

            if (nx, ny) == *obstacle {
                cell = 'O';
            }

            match cell {
                '#' | 'O' => {
                    if stop_conditions.contains(&((nx, ny), (dx, dy))) {
                        infinite_loop_counter += 1;
                        break;
                    }

                    stop_conditions.push(((nx, ny), (dx, dy)));
                    (dx, dy) = *directions.next().expect("Should produce (dx, dy)");
                }
                '.' | '^' => (x, y) = (nx, ny),
                c => panic!("Invalid symbol on the map: '{}'", c),
            }
        }
    }

    infinite_loop_counter
}

pub(crate) fn solve() -> Result<String, Box<dyn std::error::Error>> {
    let map: Vec<Vec<char>> = process_data("day006/data");
    let start: (i32, i32) = find_starting_position(&map).ok_or("Starting position not found")?;
    let guard_movement: HashSet<(i32, i32)> = record_guard_movement(&map, start);
    let result: u64 = simulate_inserting_an_obstacle(&map, start, &guard_movement);
    Ok(format!("Day 6 Guard Gallivant (Part 2): {}.", result))
}
