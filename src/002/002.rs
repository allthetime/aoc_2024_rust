use std::fs::read_to_string;

const FILENAME: &str = "data";

fn parse_input () -> Vec<Vec<i32>> {
    read_to_string(FILENAME)
        .unwrap()
        .lines()
        .map(|s| s.split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
        ).collect::<Vec<Vec<i32>>>()
}

#[derive(PartialEq)]
enum Order {
    Ascending,
    Descending,
    None
}

fn determine_order (current: i32, previous: i32) -> Order {
    if current < previous {
        return Order::Descending;
    } else if current > previous {
        return Order::Ascending;
    } else {
        return Order::None;
    }
}

fn distance_is_acceptable (current: i32, previous: i32) -> bool {
    let distance = (current - previous).abs();
    return distance <= 3 && distance > 0;
}

fn is_record_safe (record: &Vec<i32>) -> bool {
    
    let mut record_iter = record.iter().enumerate();
    let iter_len = record_iter.len();
    let last_index: usize = iter_len - 1;

    let mut last_value: Option<&i32> = None;
    let mut complete = false;
    let mut current_order = Order::None;

    while let Some((i, el)) = record_iter.next() {

        if i > 0 {
            if !distance_is_acceptable(*el, *last_value.unwrap()) { 
                break; 
            }
        }

        if i == 1 {
            match determine_order(*el, *last_value.unwrap()) {
                Order::Ascending => current_order = Order::Ascending,
                Order::Descending => current_order = Order::Descending,
                Order::None => break
            }
        }

        if i > 1 {
            if determine_order(*el, *last_value.unwrap()) != current_order { 
                break; 
            }
        }

        if i == (last_index) {
            complete = true;
        }

        last_value = Some(el);
    }

    return complete;
}

fn is_record_safe_without_element (record: Vec<i32>) -> bool {

    let record_len = record.len();

    (0..record_len).any(|i| {
        let mut record_copy = record.clone();
        record_copy.remove(i);
        return is_record_safe(&record_copy);
    })
}

fn main () {

    let reports = parse_input();

    let mut part_1_successes = 0;
    let mut part_2_successes = 0;

    for report in reports {
        part_1_successes += if is_record_safe(&report) { 1 } else { 0 };
        part_2_successes += if is_record_safe_without_element(report) { 1 } else { 0 };
    }

    println!("part1: {}\npart2: {}", part_1_successes, part_2_successes);
}