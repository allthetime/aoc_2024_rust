use std::time::Instant;
use tailcall::tailcall;

use crate::{utils, Solution};

type Id = usize;

#[derive(Debug, Copy, Clone)]
enum Builder {
    File(File),
    Space(Space),
}

#[derive(Debug, Copy, Clone)]
struct File {
    id: Id,
    length: usize,
}

#[derive(Debug, Copy, Clone)]
struct Space {
    length: usize,
}

#[derive(Debug, Copy, Clone)]
struct Block {
    value: Option<Id>,
}

#[derive(Debug, Clone)]
struct FillableSpace {
    original_size: usize,
    space_left: usize,
    files: Vec<File>,
    full: bool,
}

fn block_builder(mut blocks: Vec<Block>, builder: Builder, index: usize) -> Vec<Block> {
    let length = match &builder {
        Builder::File(file) => file.length,
        Builder::Space(space) => space.length,
    };

    if index == length {
        return blocks;
    }

    blocks.push(match &builder {
        Builder::File(file) => Block {
            value: Some(file.id),
        },
        Builder::Space(_) => Block { value: None },
    });

    block_builder(blocks, builder, index + 1)
}

fn file_system_organizer(file_system: &[Block]) -> Vec<Block> {
    let (empty_blocks_count, non_empty_blocks_count) = count_blocks(&file_system);
    let blocks_to_add = get_n_non_empty_blocks_from_back(&file_system, empty_blocks_count);

    let mut organized_file_system: Vec<Block> = Vec::new();

    #[tailcall]
    fn organizer(
        organized_file_system: &mut Vec<Block>,
        file_system: &[Block],
        blocks_to_add: &[Block],
        non_empty_blocks_count: usize,
    ) -> Vec<Block> {
        if organized_file_system.len() == non_empty_blocks_count {
            return organized_file_system.to_vec();
        }

        let first_file_system_block = file_system[0];
        let rest_of_file_system = &file_system[1..];

        if first_file_system_block.value.is_some() {
            organized_file_system.push(first_file_system_block);
            organizer(
                organized_file_system,
                rest_of_file_system,
                blocks_to_add,
                non_empty_blocks_count,
            )
        } else {
            let first_block_to_add = blocks_to_add[0];
            let rest_of_blocks_to_add = &blocks_to_add[1..];
            organized_file_system.push(first_block_to_add);
            organizer(
                organized_file_system,
                rest_of_file_system,
                rest_of_blocks_to_add,
                non_empty_blocks_count,
            )
        }
    }

    organizer(
        &mut organized_file_system,
        &file_system,
        &blocks_to_add,
        non_empty_blocks_count,
    )
}

fn print_file_system(file_system: &[Block]) {
    for block in file_system {
        match block.value {
            Some(id) => {
                print!("{}", id);
            }
            None => {
                print!(".");
            }
        }
    }
}

fn count_blocks(file_system: &[Block]) -> (usize, usize) {
    let (empty_blocks, non_empty_blocks): (Vec<&Block>, Vec<&Block>) =
        file_system.iter().partition(|block| block.value.is_none());
    (empty_blocks.len(), non_empty_blocks.len())
}

fn get_n_non_empty_blocks_from_back(file_system: &[Block], n: usize) -> Vec<Block> {
    file_system
        .iter()
        .rev()
        .filter(|block| block.value.is_some())
        .take(n)
        .map(|block| *block)
        .collect()
}

pub fn solve() -> Solution {
    let data = parse_input();
    let mut solution = Solution::new("day 9");
    let timer = utils::Timer::new();

    let file_system_builder: Vec<Builder> = data
        .into_iter()
        .enumerate()
        .map(|(i, x)| {
            let even_index = i % 2 == 0;
            let length: usize = x.parse().unwrap();
            if even_index {
                Builder::File(File {
                    id: if i == 0 { 0 } else { i / 2 },
                    length,
                })
            } else {
                Builder::Space(Space { length })
            }
        })
        .collect();

    //
    // Part 1
    //

    let file_system: Vec<Block> = file_system_builder
        .iter()
        .flat_map(|builder| block_builder(Vec::new(), *builder, 0).into_iter())
        .collect();

    let organized_file_system = file_system_organizer(&file_system);

    timer.print_elapsed();

    organized_file_system
        .iter()
        .enumerate()
        .for_each(|(index, block)| {
            if let Some(id) = block.value {
                solution.increment(utils::Part::One, index * id)
            }
        });

    //
    // Part 2
    //

    let mut spaces: Vec<(FillableSpace, usize)> = file_system_builder
        .iter()
        .enumerate()
        .filter_map(|(position, builder)| match builder {
            Builder::Space(space) => {
                if space.length > 0 {
                    Some((
                        FillableSpace {
                            original_size: space.length,
                            space_left: space.length,
                            files: Vec::new(),
                            full: false,
                        },
                        position,
                    ))
                } else {
                    None
                }
            }
            _ => None,
        })
        .collect();

    let files: Vec<(File, usize)> = file_system_builder
        .iter()
        .enumerate()
        .filter_map(|(position, builder)| match builder {
            Builder::File(file) => Some((*file, position)),
            _ => None,
        })
        .collect();

    files.iter().rev().for_each(|file| {
        let file_size = file.0.length;
        let mut space_index = 0;
        let mut available_spaces: Vec<&mut (FillableSpace, usize)> = spaces
            .iter_mut()
            .filter(|(space, _)| !space.full && space.space_left >= file_size)
            .collect();
        if !available_spaces.is_empty() {
            available_spaces[0].0.files.push(file.0);
            available_spaces[0].0.space_left -= file_size;
            if available_spaces[0].0.space_left == 0 {
                available_spaces[0].0.full = true;
            }
        }
    });

    let mut organized_file_system: Vec<File> = Vec::new();

    // dbg!(&files);
    // dbg!(&spaces);

    let mut insert_index: usize = 0;
    let mut added_file_ids: Vec<usize> = Vec::new();
    while organized_file_system.len() < files.len() {
        let is_file_index = insert_index % 2 == 0;
        let is_space_index = insert_index % 2 != 0;

        let file_index = insert_index / 2;
        let space_index = (insert_index as f32 / 2.0 - 0.5) as usize;

        if (is_file_index) {
            // dbg!(file_index);
            let file = files[file_index];
            if added_file_ids.contains(&file.0.id) {
                // do nothing
            } else {
                organized_file_system.push(file.0);
                added_file_ids.push(file.0.id);
            }
        } else if (is_space_index) {
            // dbg!(space_index);
            let (space, _) = &spaces[space_index];
            if !space.files.is_empty() {
                space.files.clone().iter().for_each(|file| {
                    if added_file_ids.contains(&file.id) {
                        // do nothing
                    } else {
                        organized_file_system.push(*file);
                        added_file_ids.push(file.id);
                    }
                });
            }
            if space.space_left > 0 {
                // dbg!(space);
                // organized_file_system.push(File {
                //     id: 9999999999999,
                //     length: space.space_left,
                // });
            }
        }

        insert_index += 1;
    }

    // dbg!(&organized_file_system);

    let final_file_system: Vec<Block> = organized_file_system
        .iter()
        .flat_map(|builder| block_builder(Vec::new(), Builder::File(*builder), 0).into_iter())
        .collect();

    // dbg!(&final_file_system);

    // print_file_system(&final_file_system);

    // final_file_system
    //     .iter()
    //     .enumerate()
    //     .for_each(|(index, block)| {
    //         if let Some(id) = block.value {
    //             solution.increment(utils::Part::Two, index * id)
    //         }
    //     });

    solution
}

fn parse_input() -> Vec<String> {
    utils::get_data_string(9, utils::DataSource::Test)
        .split("")
        .map(String::from)
        .filter(|x| !x.is_empty())
        .collect()
}
