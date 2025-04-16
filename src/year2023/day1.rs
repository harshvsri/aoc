use std::fs;

fn get_file_content() -> String {
    fs::read_to_string("input.txt").expect("input.txt must be present in the root.")
}

fn _digit_calib_value() {
    let digit_calib_value = get_file_content()
        .lines()
        .map(|line| {
            let digits = line
                .chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<Vec<_>>();

            assert!(digits.len() > 0, "No digits were found.");
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum::<u32>();

    println!("Digit Calibration Value: {:?}", digit_calib_value);
}

fn text_calib_value() {
    let text_calib_value = get_file_content()
        .lines()
        .map(|line| {
            let mut digits = Vec::new();

            let mut index = 0;
            while index < line.len() {
                let mut skip_count = 1;

                if let Some(digit) = &line
                    .chars()
                    .nth(index)
                    .expect("Must be a valid index.")
                    .to_digit(10)
                {
                    digits.push(digit.clone());
                } else {
                    for next in 2..=4 {
                        if index + next < line.len() {
                            if let Some(digit) = get_digit_from_text(&line[index..=index + next]) {
                                digits.push(digit);
                                skip_count = next;
                                break;
                            }
                        }
                    }
                }
                index += skip_count;
            }
            assert!(digits.len() > 0, "No digits were found.");

            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum::<u32>();

    println!("Text: {:?}", text_calib_value);
}

fn get_digit_from_text(text: &str) -> Option<u32> {
    match text {
        "zero" => Some(0),
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

pub fn get_calib_value() {
    text_calib_value();
}
