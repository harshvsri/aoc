#[derive(Debug)]
pub enum Command<'a> {
    CD(&'a str),
    LS(&'a str),
}

impl<'a> Command<'a> {
    pub fn parse(s: &'a str) -> Vec<Command<'a>> {
        s.split("$ ")
            .filter(|cmd| !cmd.is_empty())
            .map(|cmd| match &cmd[..2] {
                "cd" => Command::CD(&cmd[3..].trim()),
                "ls" => Command::LS(&cmd[3..].trim()),
                _ => unreachable!(),
            })
            .collect::<Vec<_>>()
    }
}

const DISK_SIZE: usize = 70_000_000;
const REQUIRED_SIZE: usize = 30_000_000;

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let cmds = Command::parse(&data);
    let mut path_stack = Vec::new();

    let mut dir_size_map = std::collections::HashMap::from([("/".to_string(), 0)]);
    for cmd in &cmds {
        // println!("Stack: {:?}", path_stack);
        match cmd {
            Command::CD(path) => match *path {
                "/" => path_stack.clear(),
                ".." => {
                    path_stack.pop();
                }
                _ => path_stack.push(*path),
            },
            Command::LS(content) => {
                content
                    .lines()
                    .filter(|line| !line.starts_with("dir"))
                    .for_each(|file_info| {
                        let (file_size, _) = file_info
                            .split_once(" ")
                            .expect("Must contain a valid delimiter");
                        let file_size = file_size.parse::<usize>().expect("Must be a valid number");

                        // Now i need to add this file in all the directories.
                        dir_size_map
                            .entry("/".to_string())
                            .and_modify(|size| *size += file_size);

                        for i in 1..=path_stack.len() {
                            let dir_path = format!("/{}", path_stack[..i].join("/"));
                            dir_size_map
                                .entry(dir_path)
                                .and_modify(|size| *size += file_size)
                                .or_insert(file_size);
                        }
                    })
            }
        }
    }

    let used_space = dir_size_map["/"];
    let free_space = DISK_SIZE - used_space;
    let needed_space = REQUIRED_SIZE.saturating_sub(free_space);

    if needed_space == 0 {
        println!("Available enough space...");
        return;
    }

    let mut dir_to_delete = used_space;
    for (_, &dir_size) in &dir_size_map {
        if dir_size >= needed_space {
            dir_to_delete = dir_to_delete.min(dir_size);
        }
    }
    println!("Directory size: {dir_to_delete}");
}
