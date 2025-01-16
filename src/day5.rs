use std::{collections::HashSet, fs};

pub fn get_data() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let (page_rules, page_numbers) = content
        .split_once("\n\n")
        .expect("There should be a \n\n present at the split.");

    let page_rules = page_rules
        .lines()
        .map(|line| {
            let (a, b) = line
                .split_once("|")
                .expect("There should be a valid seperator.");
            (
                a.parse::<i32>()
                    .expect("There should be a valid number to parse"),
                b.parse::<i32>()
                    .expect("There should be a valid number to parse"),
            )
        })
        .collect::<HashSet<(i32, i32)>>();

    let page_numbers = page_numbers
        .lines()
        .map(|line| {
            let values = line
                .split(",")
                .map(|value| {
                    value
                        .parse::<i32>()
                        .expect("THere should be a valid number to parse.")
                })
                .collect::<Vec<i32>>();
            values
        })
        .collect::<Vec<Vec<i32>>>();

    let mut res = 0;
    let mut fixed_res = 0;
    for pages in &page_numbers {
        let mut right_order = true;

        for (index, page) in pages.iter().enumerate() {
            if !is_valid(index, *page, &pages, &page_rules) {
                // We just found a wrong ordered pages.
                right_order = false;
                fixed_res += fix_pages(pages, &page_rules);
                break;
            }
        }
        if right_order {
            res += pages[pages.len() / 2];
        }
    }

    println!("Res: {}", res);
    println!("Fixed Res: {}", fixed_res);
}

fn is_valid(index: usize, page: i32, pages: &Vec<i32>, page_rules: &HashSet<(i32, i32)>) -> bool {
    for idx in 0..index {
        let pair = (page, pages[idx]);
        if page_rules.contains(&pair) {
            return false;
        }
    }

    return true;
}

fn fix_pages(pages: &Vec<i32>, page_rules: &HashSet<(i32, i32)>) -> i32 {
    let mut pages = pages.clone();
    for index in 1..pages.len() {
        let mut idx = index;
        loop {
            if index <= 0 {
                break;
            }

            let pair = (pages[idx], pages[idx - 1]);
            if page_rules.contains(&pair) {
                // Swap the indices.
                let temp = pages[idx];
                pages[idx] = pages[idx - 1];
                pages[idx - 1] = temp;
                idx -= 1;
            } else {
                break;
            }
        }
    }
    return pages[pages.len() / 2];
}
