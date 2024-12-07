use std::fs::read_to_string;
use std::collections::HashMap;

const FILENAME: &str = "data";
const FILENAME_: &str = "test.dat";

fn get_usize (s: &str) -> usize {
    s.parse::<usize>().unwrap()
}

fn parse_input () -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let file_content = read_to_string(FILENAME_).unwrap();

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

pub fn solve () -> usize {
    let (page_order_rules, page_data) = parse_input();
    // println!("{:?} {:?}", page_order_rules, page_data);

    let mut comes_after: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut comes_before: HashMap<usize, Vec<usize>> = HashMap::new();

    for (a, b) in page_order_rules {
        let entry = comes_after
            .entry(a)
            .or_insert(Vec::new());
        entry.push(b);

        let entry = comes_before
            .entry(b)
            .or_insert(Vec::new());
        entry.push(a);
    }

    // dbg!(&comes_after);
    // dbg!(comes_before);

    let mut total_of_middle_num = 0;

    for (_page_num, page) in page_data.iter().enumerate() {
        // dbg!(&page);

        let page_len: usize = page.len();
        let mut fixed_page = page.clone();
        let mut bad_page = false;
    
        for (i, &elem) in page.iter().enumerate() {
            let has_comes_after = comes_after.contains_key(&elem);
            if (has_comes_after) {
                let comes_after_pages = comes_after.get(&elem).unwrap();
                for &page_after in comes_after_pages {
                    let comes_after_index = page.iter().position(|&e| e == page_after);
                    match comes_after_index {
                        Some(comes_after_index) => {
                            let comes_before = i > comes_after_index;
                            if comes_before {
                                bad_page = true;
                                fixed_page.swap(i, comes_after_index);
                            }
                        },
                        None => {

                        }
                    }
                }              
            }
        }

        dbg!(&fixed_page, &page);

        if !bad_page {
            let middle_element_index = (page_len as f32 / 2_f32).floor() as usize;
            let middle_element = page[middle_element_index];
            total_of_middle_num += middle_element;
        }
    }

    total_of_middle_num
}

