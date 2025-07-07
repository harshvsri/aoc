use crate::lcm;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug, PartialEq, Clone)]
enum Pulse {
    Low,
    High,
    Dead,
}

impl Pulse {
    fn invert(&self) -> Self {
        match self {
            Pulse::Low => Pulse::High,
            Pulse::High => Pulse::Low,
            Pulse::Dead => panic!("Cant invert a dead pulse."),
        }
    }

    fn from_bool(val: &bool) -> Self {
        match val {
            true => Pulse::High,
            false => Pulse::Low,
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, Pulse>),
}
impl ModuleType {
    fn is_conjunction(&self) -> bool {
        match self {
            ModuleType::Conjunction(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct Module {
    module_type: ModuleType,
    destination_modules: Vec<String>,
}

impl Module {
    fn new(mod_str: &str) -> (String, Self) {
        let (mod_info, dest) = mod_str
            .split_once(" -> ")
            .expect("Must contain valid delimeter.");

        let destination_modules = dest
            .split(",")
            .map(|dest| dest.trim().to_string())
            .collect::<Vec<_>>();

        if mod_info == "broadcaster" {
            (
                mod_info.to_string(),
                Module {
                    module_type: ModuleType::Broadcaster,
                    destination_modules,
                },
            )
        } else {
            let module_type = match mod_info.chars().nth(0).unwrap() {
                '%' => ModuleType::FlipFlop(false),
                '&' => ModuleType::Conjunction(HashMap::new()),
                _ => panic!("Invalid module type found."),
            };

            let module_key = mod_info[1..].to_string();
            (
                module_key,
                Module {
                    module_type,
                    destination_modules,
                },
            )
        }
    }

    fn handle_pulse(&mut self, source: &str, mut pulse: Pulse) -> Pulse {
        match &mut self.module_type {
            ModuleType::FlipFlop(val) => {
                if pulse == Pulse::Low {
                    *val = !(*val);
                    pulse = Pulse::from_bool(val);
                } else {
                    pulse = Pulse::Dead
                }
            }
            ModuleType::Conjunction(source_modules) => {
                source_modules.insert(source.to_string(), pulse.clone());
                pulse =
                    Pulse::from_bool(&source_modules.iter().all(|(_, val)| *val == Pulse::High))
                        .invert();
            }
            ModuleType::Broadcaster => {}
        };

        pulse
    }
}

pub fn count_pulse() {
    let mut modules_map = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
        .lines()
        .map(|line| Module::new(line))
        .collect::<HashMap<_, _>>();

    let conjunction_modules = modules_map
        .iter()
        .filter(|(_, val)| val.module_type.is_conjunction())
        .map(|(key, _)| key.to_string())
        .collect::<HashSet<_>>();

    let mut modules_pairs = vec![];
    for (key, val) in &modules_map {
        for module in &val.destination_modules {
            if conjunction_modules.contains(module) {
                modules_pairs.push((key.to_string(), module.to_string()));
            }
        }
    }

    for (src, dest) in &modules_pairs {
        let module = modules_map.get_mut(dest).expect("Must be a valid key.");
        if let ModuleType::Conjunction(src_modules) = &mut module.module_type {
            src_modules.insert(src.to_string(), Pulse::Low);
        }
    }

    let (dest_module_name, dest_module) = modules_map
        .iter()
        .filter(|(_, val)| val.destination_modules.contains(&"rx".to_string()))
        .map(|(key, val)| (key.to_string(), val.clone()))
        .next()
        .expect("Iterator must not be empty.");

    println!("{dest_module_name} -> {dest_module:?}");

    let mut queue = VecDeque::new();
    let mut button_lenghts = vec![];

    if let ModuleType::Conjunction(map) = &dest_module.module_type {
        for src_module_name in map.keys() {
            let mut mod_found = false;
            let mut button_press = 0;
            let mut map = modules_map.clone();

            loop {
                if mod_found {
                    break;
                }
                button_press += 1;
                queue.push_back(("button".to_string(), Pulse::Low, "broadcaster".to_string()));

                while !queue.is_empty() {
                    let (source, pulse, destination) = queue.pop_front().unwrap();

                    if pulse == Pulse::High
                        && source == *src_module_name
                        && destination == *dest_module_name
                    {
                        println!("{src_module_name} -> {dest_module_name} [{button_press}]");
                        mod_found = true;
                        button_lenghts.push(button_press);
                        break;
                    }

                    if let Some(module) = map.get_mut(&destination) {
                        let output_pulse = module.handle_pulse(&source, pulse);
                        if output_pulse == Pulse::Dead {
                            continue;
                        }

                        for dest_module in &module.destination_modules {
                            queue.push_back((
                                destination.to_string(),
                                output_pulse.clone(),
                                dest_module.to_string(),
                            ));
                        }
                    }
                }
            }
        }

        let res = button_lenghts.iter().cloned().reduce(lcm).unwrap();
        println!("Res -> {res}");
    }
}
