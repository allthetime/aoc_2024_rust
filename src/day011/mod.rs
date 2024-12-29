use core::num;
use std::{collections::HashMap, thread};

use crate::{utils, Solution};

const LIMIT: usize = 75;

enum Number {
    Zero,
    Even(i64, i64),
    Odd(i64),
}

//
// way faster than all other methods using lists
// COUNT occurences in hash
//
fn true_cache_method(string: String) -> i64 {
    let mut cache: HashMap<i64, i64> = HashMap::new();
    let starting_number = string.parse::<i64>().unwrap();

    cache.insert(starting_number, 1);

    (0..LIMIT).for_each(|iteration| {
        let mut new_cache: HashMap<i64, i64> = HashMap::new();
        for (number, count) in cache.iter() {
            match number {
                0 => *new_cache.entry(1).or_default() += count,
                x if number_of_digits(*x) % 2 == 0 => {
                    let (left, right) = split_number_in_half_mathematically(*x);
                    *new_cache.entry(left).or_default() += count;
                    *new_cache.entry(right).or_default() += count;
                }
                _ => {
                    *new_cache.entry(number * 2024).or_default() += count;
                }
            }
        }
        cache = new_cache;
    });

    cache.values().sum::<i64>()
}

fn build_numbers(string: String) -> Vec<i64> {
    let mut numbers = vec![string.parse::<i64>().unwrap()];
    let mut cache: HashMap<i64, Number> = HashMap::new();

    (0..LIMIT).for_each(|_| {
        let mut new_numbers = vec![];
        while let Some(s) = numbers.pop() {
            if cache.contains_key(&s) {
                match cache.get(&s).unwrap() {
                    Number::Zero => continue,
                    Number::Even(left, right) => {
                        new_numbers.push(*left);
                        new_numbers.push(*right);
                    }
                    Number::Odd(n) => new_numbers.push(*n),
                }
                continue;
            }

            let new_s = match s {
                0 => {
                    cache.insert(s, Number::Odd(1));
                    Number::Odd(1)
                }
                x if number_of_digits(x) % 2 == 0 => {
                    let (left, right) = split_number_in_half_mathematically(x);
                    cache.insert(s, Number::Even(left, right));
                    Number::Even(left, right)
                }
                _ => {
                    cache.insert(s, Number::Odd(s * 2024));
                    Number::Odd(s * 2024)
                }
            };

            match new_s {
                Number::Zero => continue,
                Number::Even(left, right) => {
                    new_numbers.push(left);
                    new_numbers.push(right);
                }
                Number::Odd(n) => new_numbers.push(n),
            }
        }
        numbers = new_numbers;
    });

    numbers
}

fn number_of_digits(n: i64) -> i64 {
    (n as f64).log10().floor() as i64 + 1
}

fn split_number_in_half_mathematically(n: i64) -> (i64, i64) {
    let digits = number_of_digits(n);
    let half = digits / 2;
    let left = n / 10_i64.pow(half as u32);
    let right = n % 10_i64.pow(half as u32);
    (left, right)
}

pub fn solve() -> Solution {
    let data = parse_input();
    let mut solution = Solution::new("day 011");
    let timer = utils::Timer::new();

    // let LIMIT: u32 = 6;

    let threads = data
        .into_iter()
        .enumerate()
        .map(|(index, string)| thread::spawn(move || true_cache_method(string)))
        .collect::<Vec<_>>();
    // try without collect and see what happens (makes it syncronous)

    // for t in threads {
    //     let res = t.join().unwrap();
    //     dbg!(&res);
    // }

    // let vals = threads
    //     .into_iter()
    //     .flat_map(|t| t.join().unwrap())
    //     // .flatten()
    //     .collect::<Vec<_>>();

    // let length = vals.len();

    let vals = threads.into_iter().fold(0, |mut acc, t| {
        acc += t.join().unwrap();
        acc
    });

    timer.print_elapsed();
    solution.increment(utils::Part::One, vals as usize);
    solution
}

fn parse_input() -> Vec<String> {
    utils::get_data_string(11, utils::DataSource::Data)
        .split_whitespace()
        .map(String::from)
        .collect()
}
