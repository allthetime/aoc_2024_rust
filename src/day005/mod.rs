use std::fs::read_to_string;
use std::collections::HashMap;
use crate::utils::{get_data_string, DataSource, Part, Solution};

fn get_usize (s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn parse_input () -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let file_content = get_data_string(5,DataSource::Data);

    let data: Vec<&str> = file_content
        .split("\n\n")
        .collect();

    let page_order_rules: Vec<(usize, usize)> = data[0]
            .split("\n")
            .map(|e| e
                .split_once("|").map(|(s1,s2)| (
                    get_usize(s1), get_usize(s2)
                )).unwrap()                
            ).collect();

    let page_data: Vec<Vec<usize>> = data[1]            
        .split("\n")
            .map(|e| e
                .split(",")
                .map(get_usize).collect()
            ).collect();
    
    (page_order_rules, page_data)
}

fn get_middle_element (page: &Vec<usize>) -> usize {
    let page_len: usize = page.len();
    let middle_element_index = (page_len as f32 / 2_f32).floor() as usize;
    page[middle_element_index]
}

fn build_order_map (page_order_rules: &Vec<(usize, usize)>, comes_after: &mut HashMap<usize, Vec<usize>>) {
    for (a, b) in page_order_rules {
        let entry = comes_after
            .entry(*a)
            .or_insert(Vec::new());
        entry.push(*b);
    }
}

pub fn solve () -> Solution {

    let mut solution = Solution::new("day 5");

    let (page_order_rules, mut page_data) = parse_input();
    
    let mut comes_after: HashMap<usize, Vec<usize>> = HashMap::new();
    build_order_map(&page_order_rules, &mut comes_after);

    for page in page_data.iter_mut() {
        
        let mut page_out_of_order = false;
    
        for (i, &elem) in page.iter().enumerate() {
            if comes_after.contains_key(&elem) {
                for &comes_after_elem in comes_after.get(&elem).unwrap() {
                    page_out_of_order = match page.iter().position(|&e| e == comes_after_elem) {
                        Some(comes_after_index) => i > comes_after_index || page_out_of_order,
                        None => page_out_of_order
                    }
                }              
            }
        }

        if !page_out_of_order {
            solution.increment(Part::One, get_middle_element(page))
        } else {
            page.sort_by(|a, b| {
                match comes_after.get(&b) {
                    Some(after_nums) => if after_nums.iter().any(|&n| n == *a) {
                            std::cmp::Ordering::Less
                        } else {
                            std::cmp::Ordering::Greater
                        },
                    None=> std::cmp::Ordering::Equal
                }
            });
            solution.increment(Part::Two, get_middle_element(&page))
        }
    }

    solution
}

