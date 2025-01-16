use std::fs;

use regex::Regex;

pub fn _parse_input() {
    let content = fs::read_to_string("input.txt")
        .expect("input.txt should be present in the root directory.");

    let pattern = r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don\'t\(\)";
    let regex = Regex::new(pattern).expect("Input pattern should be valid.");

    let mut res = 0;
    let mut allow = true;

    for line in content.lines() {
        for mat in regex.find_iter(line).map(|m| m.as_str()) {
            res += match mat {
                "do()" => {
                    allow = true;
                    0
                }
                "don't()" => {
                    allow = false;
                    0
                }
                _ if allow => _calculate(mat),
                _ => 0,
            };
        }
    }
    println!("Res: {}", res);
}

fn _calculate(s: &str) -> i32 {
    let nums = s[4..s.len() - 1]
        .split(",")
        .map(|num| num.parse::<i32>().expect("Expected valid numbers"))
        .collect::<Vec<i32>>();

    return nums[0] * nums[1];
}
