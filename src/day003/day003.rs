use std::fs::read_to_string;

const FILENAME: &str = "data";

fn parse_input () -> String {
    read_to_string(FILENAME)
        .unwrap()
}

fn part_1 (data: String) -> () {
    let data = data
        .split("mul(")
        .map(String::from)
        .map(
            |s| s.split(")")
                .next()
                .unwrap()
                .to_string()
        ).collect::<Vec<String>>();

    let data = &data
        .iter()
        .map(
            |s| s.split(",")
                .collect::<Vec<_>>()
        )
        .filter(|e| e.len() == 2)
        .filter(|e| e.iter().all(
            |f| match f.parse::<i32>() {
                Ok(_)=> true,
                Err(__)=> false
            }
        )).collect::<Vec<_>>();

    let mut data = &data.iter()
        .map(|e| e.iter()
            .map(|f| f.parse::<i32>().unwrap()
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let mut total = 0;

    for pair in data  {
        let product = pair.iter().copied().reduce(|a,b| a*b).unwrap();
        total += product;
    }

    println!("part1: {total}");
}

pub fn main() {

    let data = parse_input();

    part_1(data.clone());

    let mut multing = false;
    let mut dont_do_it = false;
    let mut cursor: usize = 0;
    let chars_length = data.chars().count();

    let mut total_2 = 0;

    while cursor < chars_length {

        let current_char = data.chars().nth(cursor).unwrap();

        match current_char {
            'm' => {
                let next_four = data.chars().skip(cursor).take(4).collect::<String>();
                if !dont_do_it {
                    multing = next_four == "mul(";
                    cursor += if multing { 4 } else { 1 };   
                } else {
                    cursor += 1;
                }
            },
            'd' => {
                let next_seven = data.chars().skip(cursor).take(7).collect::<String>();
                let do_ = &next_seven[0..4] == "do()";
                let dont_ = &next_seven == "don't()";
                if dont_ { 
                    dont_do_it = true; 
                    cursor += 7;
                    continue;
                }
                if do_ && dont_do_it { 
                    dont_do_it = false; 
                    cursor += 4;
                    continue;
                }
                cursor += 1;
            },
            _ => {
                if multing && !dont_do_it {
                    let until_close = data.chars().skip(cursor).take_while(|c| *c != ')').collect::<String>();
                    if until_close.chars().count() > 7 {
                        cursor += 1;
                        multing = false;
                    } else {
                        let pair = until_close.split(",").collect::<Vec<_>>();
                        if pair.len() == 2 {
                                let both_ints = pair.iter().all(
                                    |f| match f.parse::<i32>() {
                                        Ok(_)=> true,
                                        Err(__)=> false
                                    }
                                );
                                if both_ints {
                                    let pair = pair.iter().map(|e| e.parse::<i32>().unwrap()).collect::<Vec<_>>();
                                    let product = pair.iter().copied().reduce(|a,b| a*b).unwrap();
                                    total_2 += product;
                                    cursor += until_close.chars().count();
                                    multing = false;
                                } else {
                                    cursor += 1;
                                    multing = false;
                                }
                        } else {
                            cursor += 1;
                            multing = false;
                        }
                    }
                } else {
                    cursor += 1;
                }
            }
        }
    }

    println!("part2: {}", total_2);

    return;


}