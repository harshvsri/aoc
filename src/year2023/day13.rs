#[derive(Debug, PartialEq, PartialOrd)]
enum ReflectionAxis {
    Row(usize, usize),
    Col(usize, usize),
}

impl ReflectionAxis {
    fn to_notes(&self) -> usize {
        match self {
            ReflectionAxis::Row(x, _) => 100 * (*x + 1),
            ReflectionAxis::Col(x, _) => *x + 1,
        }
    }
}

pub fn get_reflection_notes() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let pattern_notes = content
        .split("\n\n")
        .map(|pattern| {
            let mut pattern = pattern
                .lines()
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>();

            let org_axis = get_reflection_axis(&pattern, None)
                .expect("At least one reflection axis must be present.");
            let mut new_axis = None;

            for row in 0..pattern.len() {
                for col in 0..pattern[0].len() {
                    let ch = pattern[row][col];
                    match ch {
                        '.' => pattern[row][col] = '#',
                        '#' => pattern[row][col] = '.',
                        _ => panic!("Invalid character found."),
                    }
                    if let Some(axis) = get_reflection_axis(&pattern, Some(&org_axis)) {
                        if let Some(prev_axis) = &new_axis {
                            if *prev_axis != axis {
                                panic!("We found 2 new reflection axis.")
                            }
                        } else {
                            new_axis = Some(axis);
                        }
                    }
                    pattern[row][col] = ch;
                }
            }

            new_axis.expect("Must have atleast one new axis").to_notes()
        })
        .sum::<usize>();

    println!("Pattern notes -> {pattern_notes}");
}

fn get_reflection_axis(
    pattern: &Vec<Vec<char>>,
    prev_axis: Option<&ReflectionAxis>,
) -> Option<ReflectionAxis> {
    let ranges = [(0, pattern.len()), (0, pattern[0].len())];
    let mut axis = None;

    for (index, &(start, end)) in ranges.iter().enumerate() {
        (start..end)
            .map(|val| val)
            .collect::<Vec<_>>()
            .windows(2)
            .for_each(|window| {
                let (a, b) = (window[0], window[1]);

                match index {
                    0 => {
                        if has_reflection_along_row(pattern, a as isize, b as isize) {
                            if let Some(p_axis) = prev_axis {
                                if *p_axis != ReflectionAxis::Row(a, b) {
                                    axis = Some(ReflectionAxis::Row(a, b));
                                }
                            } else {
                                axis = Some(ReflectionAxis::Row(a, b));
                            }
                        }
                    }
                    1 => {
                        if has_reflection_along_col(pattern, a as isize, b as isize) {
                            if let Some(p_axis) = prev_axis {
                                if *p_axis != ReflectionAxis::Col(a, b) {
                                    axis = Some(ReflectionAxis::Col(a, b));
                                }
                            } else {
                                axis = Some(ReflectionAxis::Col(a, b));
                            }
                        }
                    }
                    _ => panic!("Invalid index."),
                }
            })
    }

    return axis;
}

fn has_reflection_along_row(pattern: &Vec<Vec<char>>, mut top: isize, mut down: isize) -> bool {
    while top >= 0 && down < pattern.len() as isize {
        if pattern[top as usize] != pattern[down as usize] {
            return false;
        }
        top -= 1;
        down += 1;
    }
    return true;
}

fn has_reflection_along_col(pattern: &Vec<Vec<char>>, mut left: isize, mut right: isize) -> bool {
    while left >= 0 && right < pattern[0].len() as isize {
        for row in 0..pattern.iter().len() {
            if pattern[row][left as usize] != pattern[row][right as usize] {
                return false;
            }
        }
        left -= 1;
        right += 1;
    }
    return true;
}
