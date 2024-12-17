use crate::{
    utils::{self, Part},
    Solution,
};
use std::collections::{HashMap, HashSet};

fn parse_input() -> Vec<Vec<char>> {
    utils::get_data_string(8, utils::DataSource::Data)
        .lines()
        .map(|x| x.chars().collect::<Vec<char>>())
        .collect()
}

#[derive(Debug)]
struct Antenna {
    position: (isize, isize),
    char_: char,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Antinode {
    position: (isize, isize),
}

fn get_distance_between_positions(a: (isize, isize), b: (isize, isize)) -> (isize, isize) {
    let (x1, y1) = a;
    let (x2, y2) = b;
    (x1 - x2, y1 - y2)
}

fn get_antinodes(
    a: (isize, isize),
    b: (isize, isize),
    harmonic: isize,
) -> ((isize, isize), (isize, isize)) {
    let (dx, dy) = get_distance_between_positions(a, b);
    (
        (a.0 + dx * harmonic, a.1 + dy * harmonic),
        (b.0 - dx * harmonic, b.1 - dy * harmonic),
    )
}

fn position_is_in_bounds(position: (isize, isize), width: usize, height: usize) -> bool {
    let (x, y) = position;
    x >= 0 && x < width as isize && y >= 0 && y < height as isize
}

fn get_antinodes_for_all_harmonics_in_bounds(
    harmonic: isize,
    mut antinodes_vec: Vec<Antinode>,
    position_a: (isize, isize),
    position_b: (isize, isize),
    height: usize,
    width: usize,
    stop_at: Option<isize>,
) -> Vec<Antinode> {
    let (antinode1, antinode2) = get_antinodes(position_a, position_b, harmonic);
    let a1_in_bounds = position_is_in_bounds(antinode1, width, height);
    let a2_in_bounds = position_is_in_bounds(antinode2, width, height);

    if a1_in_bounds {
        antinodes_vec.push(Antinode {
            position: antinode1,
        });
    }

    if a2_in_bounds {
        antinodes_vec.push(Antinode {
            position: antinode2,
        });
    }

    if !(a1_in_bounds || a2_in_bounds) || stop_at == Some(harmonic) {
        return antinodes_vec;
    }

    get_antinodes_for_all_harmonics_in_bounds(
        harmonic + 1,
        antinodes_vec,
        position_a,
        position_b,
        height,
        width,
        stop_at,
    )
}

fn get_antinodes_for_antenna(
    mut current: Vec<Antinode>,
    antennas: &[Antenna],
    width: usize,
    height: usize,
    stop: Option<isize>,
) -> Vec<Antinode> {
    if antennas.is_empty() {
        return current;
    }

    let first_antenna = &antennas[0];
    let rest = &antennas[1..];

    rest.iter().for_each(|antenna| {
        let antinodes = get_antinodes_for_all_harmonics_in_bounds(
            1,
            Vec::new(),
            first_antenna.position,
            antenna.position,
            height,
            width,
            stop,
        );

        current.extend(antinodes);
    });

    get_antinodes_for_antenna(current, rest, width, height, stop)
}

pub fn solve() -> Solution {
    let data = parse_input();
    let mut solution = Solution::new("day 8");

    let height = data.len();
    let width = data[0].len();

    let antenna_by_frequency = data
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter().enumerate().filter_map(move |(x, c)| match c {
                '.' => None,
                _ => Some(Antenna {
                    position: (x as isize, y as isize),
                    char_: *c,
                }),
            })
        })
        .fold(HashMap::<char, Vec<Antenna>>::new(), |mut acc, antenna| {
            acc.entry(antenna.char_).or_default().push(antenna);
            acc
        });

    let antinodes_1 = antenna_by_frequency
        .iter()
        .flat_map(|(_, antennas)| {
            get_antinodes_for_antenna(Vec::new(), antennas, width, height, Some(1))
        })
        .collect::<HashSet<_>>();

    let antinodes_2 = antenna_by_frequency
        .iter()
        .flat_map(|(_, antennas)| {
            let antinodes = get_antinodes_for_antenna(Vec::new(), antennas, width, height, None);
            let antennas_as_antinodes: Vec<Antinode> = antennas
                .iter()
                .map(|antenna| Antinode {
                    position: antenna.position,
                })
                .collect();
            [antinodes, antennas_as_antinodes].concat()
        })
        .collect::<HashSet<_>>();

    solution.increment(Part::One, antinodes_1.len());
    solution.increment(Part::Two, antinodes_2.len());

    solution
}
