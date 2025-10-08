use std::{
    cmp::{max, min},
    collections::HashMap,
};

enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

struct Workflow {
    rules: Vec<Rule>,
    destination: Destination,
}

impl Workflow {
    fn new(workflow: &str) -> Self {
        let (rules, destination) = workflow
            .rsplit_once(',')
            .expect("Workflow must include a terminal destination.");

        let destination = match destination {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            _ => Destination::Workflow(destination.to_string()),
        };

        let rules = rules.split(',').map(Rule::new).collect::<Vec<_>>();

        Workflow { rules, destination }
    }
}

struct Rule {
    variable: char,
    operator: char,
    value: usize,
    destination: Destination,
}

impl Rule {
    fn new(rule: &str) -> Self {
        let (ops, destination) = rule
            .split_once(":")
            .expect("Must contain a valid delimeter.");
        let mut ops_iter = ops.chars();

        Rule {
            variable: ops_iter.next().unwrap(),
            operator: ops_iter.next().unwrap(),
            value: (ops_iter
                .collect::<String>()
                .parse::<usize>()
                .expect("Must be a valid number."))
            .clone(),
            destination: match destination {
                "A" => Destination::Accept,
                "R" => Destination::Reject,
                _ => Destination::Workflow(destination.to_string()),
            },
        }
    }
}

struct Range {
    x: (usize, usize),
    m: (usize, usize),
    a: (usize, usize),
    s: (usize, usize),
}

impl Range {
    fn new() -> Self {
        Range {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        }
    }

    fn product(&self) -> usize {
        (self.x.1 - self.x.0 + 1)
            * (self.m.1 - self.m.0 + 1)
            * (self.a.1 - self.a.0 + 1)
            * (self.s.1 - self.s.0 + 1)
    }

    fn is_valid(&self) -> bool {
        self.x.0 <= self.x.1 && self.m.0 <= self.m.1 && self.a.0 <= self.a.1 && self.s.0 <= self.s.1
    }

    fn split(&self, rule: &Rule) -> (Range, Range) {
        fn build(range: &Range, variable: char, interval: (usize, usize)) -> Range {
            let (low, high) = interval;
            match variable {
                'x' => Range {
                    x: (low, high),
                    m: range.m,
                    a: range.a,
                    s: range.s,
                },
                'm' => Range {
                    x: range.x,
                    m: (low, high),
                    a: range.a,
                    s: range.s,
                },
                'a' => Range {
                    x: range.x,
                    m: range.m,
                    a: (low, high),
                    s: range.s,
                },
                's' => Range {
                    x: range.x,
                    m: range.m,
                    a: range.a,
                    s: (low, high),
                },
                _ => panic!("Invalid variable found."),
            }
        }

        let (low, high) = match rule.variable {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => panic!("Invalid variable found."),
        };

        let (matched_interval, rest_interval) = match rule.operator {
            '<' => (
                (low, min(high, rule.value - 1)),
                (max(low, rule.value), high),
            ),
            '>' => (
                (max(low, rule.value + 1), high),
                (low, min(high, rule.value)),
            ),
            _ => panic!("Invalid operator found."),
        };

        (
            build(self, rule.variable, matched_interval),
            build(self, rule.variable, rest_interval),
        )
    }
}

fn transform(workflow_key: &str, range: Range, workflow_map: &HashMap<String, Workflow>) -> usize {
    if !range.is_valid() || workflow_key == "R" {
        return 0;
    }

    if workflow_key == "A" {
        return range.product();
    }

    let workflow = workflow_map.get(workflow_key).unwrap();
    let mut pending = range;
    let mut total = 0;

    // Walk each explicit rule, peeling off the portion of the range that satisfies it.
    for rule in &workflow.rules {
        let (matched, rest) = pending.split(rule);

        if matched.is_valid() {
            total += match &rule.destination {
                Destination::Accept => matched.product(),
                Destination::Reject => 0,
                Destination::Workflow(next) => transform(next, matched, workflow_map),
            };
        }

        pending = rest;
        if !pending.is_valid() {
            break;
        }
    }

    // Whatever remains follows the workflow's default destination.
    if pending.is_valid() {
        total += match &workflow.destination {
            Destination::Accept => pending.product(),
            Destination::Reject => 0,
            Destination::Workflow(next) => transform(next, pending, workflow_map),
        };
    }

    total
}

pub fn process() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let workflow_map = content
        .split_once("\n\n")
        .expect("Must contain a valid delimeter.")
        .0
        .lines()
        .map(|workflow| {
            let (name, flow) = workflow
                .split_once("{")
                .expect("Must contain a valid delimeter.");
            (
                name.to_string(),
                Workflow::new(
                    flow.strip_suffix("}")
                        .expect("Must contain a valid delimeter."),
                ),
            )
        })
        .collect::<HashMap<_, _>>();

    println!(
        "Total combinations: {}",
        transform("in", Range::new(), &workflow_map)
    );
}
