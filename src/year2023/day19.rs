use std::collections::HashMap;

pub fn process() {
    let content = std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.");

    let (worflows, parts) = content
        .split_once("\n\n")
        .expect("Must contain a valid delimeter.");

    let parts = parts
        .lines()
        .map(|part| Part::new(&part[1..part.len() - 1]))
        .collect::<Vec<_>>();

    let workflow_map = worflows
        .lines()
        .map(|workflow| {
            let (name, flow) = workflow
                .split_once("{")
                .expect("Must contain a valid delimeter.");
            (name.to_string(), Workflow::new(&flow[..flow.len() - 1]))
        })
        .collect::<HashMap<_, _>>();

    let res = parts
        .iter()
        .filter(|part| part.process("in", &workflow_map) == Destination::Accept)
        .map(|part| part.get_sum())
        .sum::<usize>();
    println!("Res -> {res}");
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
    destination: Destination,
}

impl Workflow {
    fn new(workflow: &str) -> Self {
        let rules = workflow.split(",").collect::<Vec<_>>();

        let destination = match rules[rules.len() - 1] {
            "A" => Destination::Accept,
            "R" => Destination::Reject,
            _ => Destination::Workflow(rules[rules.len() - 1].to_string()),
        };

        let rules = rules[..rules.len() - 1]
            .iter()
            .map(|rule| Rule::new(rule))
            .collect::<Vec<_>>();

        Workflow { rules, destination }
    }
}

#[derive(Debug)]
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
            value: (&ops[2..].parse::<usize>().expect("Must be a valid number.")).clone(),
            destination: match destination {
                "A" => Destination::Accept,
                "R" => Destination::Reject,
                _ => Destination::Workflow(destination.to_string()),
            },
        }
    }

    fn is_valid(&self, part: &Part) -> bool {
        let value = match self.variable {
            'x' => part.x,
            'm' => part.m,
            'a' => part.a,
            's' => part.s,
            _ => panic!("Invalid character found as rule.variable"),
        };

        match self.operator {
            '<' => value < self.value,
            '>' => value > self.value,
            _ => panic!("Invalid character found as rule.operator"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum Destination {
    Accept,
    Reject,
    Workflow(String),
}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl Part {
    fn new(values: &str) -> Self {
        let vals = values
            .split(",")
            .map(|value| {
                let (_, val) = value.split_once("=").expect("Must have a valid delimeter.");
                val.parse::<usize>().expect("Must be a valid number.")
            })
            .collect::<Vec<_>>();

        assert!(vals.len() == 4, "Length of vals must be exactly 4.");
        Part {
            x: vals[0],
            m: vals[1],
            a: vals[2],
            s: vals[3],
        }
    }

    fn process(&self, workflow_key: &str, workflow_map: &HashMap<String, Workflow>) -> Destination {
        let mut workflow_key = workflow_key;
        loop {
            print!("{workflow_key} -> ");

            let workflow = workflow_map
                .get(workflow_key)
                .expect("workflow_key must be valid.");
            let mut destination = &workflow.destination;

            for rule in &workflow.rules {
                if rule.is_valid(&self) {
                    destination = &rule.destination;
                    break;
                }
            }

            if let Destination::Workflow(flow_key) = destination {
                workflow_key = flow_key;
            } else {
                println!("{:?}", destination);
                return destination.clone();
            }
        }
    }

    fn get_sum(&self) -> usize {
        self.x + self.m + self.a + self.s
    }
}
