use std::fs::read_to_string;

const FILENAME: &str = "data";

fn parse_input () -> String {
    read_to_string(FILENAME)
        .unwrap()
        // .lines()
        // .map(|s| s.split_whitespace()
        //     .map(|s| s.parse::<i32>().unwrap())
        //     .collect::<Vec<i32>>()
        // ).collect::<Vec<Vec<i32>>>()
}

fn main() {
    let data = parse_input();

    let chars_with_indexes = data.char_indices();
    let count = chars_with_indexes.count();
    let cursor = 0;

    let two = data
        .split("mul(")
        .map(String::from)
        .map(
            |s| s.split(")")
                .next()
                .unwrap()
                .to_string()
        ).collect::<Vec<String>>();

    let two = &two
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
            )
        ).collect::<Vec<_>>();

    let mut two = &two.iter()
        .map(|e| e.iter()
            .map(|f| f.parse::<i32>().unwrap()
            ).collect::<Vec<_>>()
        ).collect::<Vec<_>>();

    let mut total = 0;

    for pair in two  {
        let product = pair.iter().copied().reduce(|a,b| a*b).unwrap();
        println!("{}", product);
        total += product;
    }







    println!("{total}");

}