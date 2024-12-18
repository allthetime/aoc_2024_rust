use crate::{utils, Solution};

type Id = usize;

#[derive(Debug, Copy, Clone)]
enum Builder {
    File(File),
    Space(SpaceBuilder),
}

#[derive(Debug, Copy, Clone)]
struct File {
    id: Id,
    length: usize,
}

#[derive(Debug, Copy, Clone)]
struct SpaceBuilder {
    length: usize,
}

#[derive(Debug, Copy, Clone)]
struct Block {
    value: Option<Id>,
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

fn count_empty_blocks(file_system: &[(usize, Block)]) -> usize {
    file_system
        .iter()
        .filter(|block| block.1.value.is_none())
        .count()
}

fn count_non_empty_blocks(file_system: &[(usize, Block)]) -> usize {
    file_system
        .iter()
        .filter(|block| block.1.value.is_some())
        .count()
}

fn count_blocks(file_system: &[(usize, Block)]) -> (usize, usize) {
    let empty_blocks = count_empty_blocks(&file_system);
    let non_empty_blocks = count_non_empty_blocks(&file_system);
    (empty_blocks, non_empty_blocks)
}

fn get_n_non_empty_blocks_from_back(
    file_system: &[(usize, Block)],
    n: usize,
) -> Vec<(usize, Block)> {
    file_system
        .iter()
        .rev()
        .filter(|block| block.1.value.is_some())
        .take(n)
        .map(|(i, block)| (*i, *block))
        .collect()
}

pub fn solve() -> Solution {
    let data = parse_input();
    let mut solution = Solution::new("day 9");

    let builder: Vec<Builder> = data
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
                Builder::Space(SpaceBuilder { length })
            }
        })
        .collect();

    let file_system: Vec<(usize, Block)> = builder
        .iter()
        .flat_map(|builder| block_builder(Vec::new(), *builder, 0).into_iter())
        .enumerate()
        .collect();

    let (empty_blocks, non_empty_blocks) = count_blocks(&file_system);

    dbg!(empty_blocks, non_empty_blocks);

    let numbers_to_move = get_n_non_empty_blocks_from_back(&file_system, empty_blocks);

    let mut organized_file_system: Vec<Block> = Vec::new();

    while organized_file_system.len() < non_empty_blocks {
        let mut index = 0;
        for (i, block) in file_system.iter() {
            if block.value.is_some() {
                organized_file_system.push(*block);
            } else {
                organized_file_system.push(numbers_to_move[index].1);
                index += 1;
            }
        }
    }

    print_file_system(&organized_file_system[0..non_empty_blocks]);

    organized_file_system[0..non_empty_blocks]
        .iter()
        .enumerate()
        .for_each(|(index, block)| {
            if let Some(id) = block.value {
                solution.increment(utils::Part::One, index * id)
            }
        });

    solution
}

fn parse_input() -> Vec<String> {
    utils::get_data_string(9, utils::DataSource::Data)
        .split("")
        .map(String::from)
        .filter(|x| !x.is_empty())
        .collect()
}
