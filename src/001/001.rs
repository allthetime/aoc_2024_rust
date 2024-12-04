use std::{collections::HashMap, iter::zip, env, fs::read_to_string};

const FILENAME: &str = "data";

//
// see for inspiriation https://github.com/jayo60013/aoc_2024/blob/main/day01/src/main.rs
//

fn main() {

    let int_pairs = read_to_string(FILENAME) 
        .unwrap()  
        .lines()
        .map(|s| s.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>();

    let (mut line1, mut line2) = (Vec::new(), Vec::new());

    int_pairs.iter().for_each(|pair| {
        line1.push(pair[0]);
        line2.push(pair[1]);
    });

    line1.sort();
    line2.sort();

    let difference_pairs = zip(&line1, &line2);
    let sum_of_differences: i32 = difference_pairs.map(|(a, b)| (a - b).abs()).sum();
    
    println!("{}", sum_of_differences);

    let mut count_map_left: HashMap<i32, i32> = HashMap::new();
    let mut count_map_right: HashMap<i32, i32> = HashMap::new();

    for i in line1.iter() {
        let count = count_map_left.entry(*i).or_insert(0);
        *count += 1;
    }

    for i in line2.iter() {
        let count = count_map_right.entry(*i).or_insert(0);
        *count += 1;
    }

    let similarity_score: i32 = count_map_left
    .iter()
    .map(|(k, v)| {
        let n = count_map_right.get(k).unwrap_or(&0);
        return k * n * v;
    })
    .sum();

    println!("{}", similarity_score);

}