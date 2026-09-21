use std::collections::HashMap;

const PART_1_SIZE_LIMIT: usize = 100_000;
const DISK_SIZE: usize = 70_000_000;
const REQUIRED_SIZE: usize = 30_000_000;

enum Command<'a> {
    Cd(&'a str),
    Ls,
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let dir_sizes = directory_sizes(&data).expect("Input must be a valid terminal transcript");

    let part_1 = dir_sizes
        .values()
        .filter(|&&dir_size| dir_size <= PART_1_SIZE_LIMIT)
        .sum::<usize>();

    let used_space = dir_sizes["/"];
    let free_space = DISK_SIZE - used_space;
    let needed_space = REQUIRED_SIZE.saturating_sub(free_space);

    println!("Part 1: {part_1}");

    if needed_space == 0 {
        println!("Part 2: 0");
        return;
    }

    let part_2 = dir_sizes
        .values()
        .filter(|&&dir_size| dir_size >= needed_space)
        .min()
        .expect("At least root directory should be large enough to delete");

    println!("Part 2: {part_2}");
}

fn directory_sizes(data: &str) -> Result<HashMap<String, usize>, String> {
    let mut current_path = vec!["/".to_string()];
    let mut dir_sizes = HashMap::from([("/".to_string(), 0)]);

    for (line_index, line) in data.lines().enumerate() {
        let line_number = line_index + 1;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        if let Some(command) = line.strip_prefix("$ ") {
            match parse_command(command, line_number)? {
                Command::Cd(target) => change_directory(&mut current_path, target),
                Command::Ls => {}
            }
            continue;
        }

        if let Some(dir_name) = line.strip_prefix("dir ") {
            if dir_name.trim().is_empty() {
                return Err(format!("line {line_number}: missing directory name"));
            }
            continue;
        }

        let file_size = parse_file_size(line, line_number)?;
        for dir_path in &current_path {
            *dir_sizes.entry(dir_path.clone()).or_default() += file_size;
        }
    }

    Ok(dir_sizes)
}

fn parse_command(command: &str, line_number: usize) -> Result<Command<'_>, String> {
    let mut parts = command.split_whitespace();

    match parts.next() {
        Some("cd") => {
            let target = parts
                .next()
                .ok_or_else(|| format!("line {line_number}: missing cd target"))?;

            if parts.next().is_some() {
                return Err(format!("line {line_number}: cd command has too many arguments"));
            }

            Ok(Command::Cd(target))
        }
        Some("ls") => {
            if parts.next().is_some() {
                return Err(format!("line {line_number}: ls command has too many arguments"));
            }

            Ok(Command::Ls)
        }
        Some(command_name) => Err(format!("line {line_number}: unknown command `{command_name}`")),
        None => Err(format!("line {line_number}: missing command")),
    }
}

fn change_directory(current_path: &mut Vec<String>, target: &str) {
    match target {
        "/" => current_path.truncate(1),
        ".." => {
            if current_path.len() > 1 {
                current_path.pop();
            }
        }
        dir_name => {
            let parent = current_path.last().expect("Root path should always exist");
            let dir_path = if parent == "/" {
                format!("/{dir_name}")
            } else {
                format!("{parent}/{dir_name}")
            };

            current_path.push(dir_path);
        }
    }
}

fn parse_file_size(line: &str, line_number: usize) -> Result<usize, String> {
    let mut parts = line.split_whitespace();
    let file_size = parts
        .next()
        .ok_or_else(|| format!("line {line_number}: missing file size"))?;

    if parts.next().is_none() {
        return Err(format!("line {line_number}: missing file name"));
    }

    file_size
        .parse::<usize>()
        .map_err(|_| format!("line {line_number}: invalid file size `{file_size}`"))
}
