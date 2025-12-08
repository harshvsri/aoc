use std::collections::HashSet;

#[derive(Debug, Eq, Hash, PartialEq, PartialOrd, Ord)]
struct JunctionBox {
    x: u64,
    y: u64,
    z: u64,
}

impl JunctionBox {
    fn new(line: &str) -> Self {
        let mut parts = line.split(',');
        let x = parts.next().unwrap().parse().unwrap();
        let y = parts.next().unwrap().parse().unwrap();
        let z = parts.next().unwrap().parse().unwrap();
        JunctionBox { x, y, z }
    }

    fn distance(&self, other: &JunctionBox) -> u64 {
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }

    fn parent_circuit(&self, circuits: &[HashSet<&JunctionBox>]) -> Option<usize> {
        circuits.iter().position(|set| set.contains(self))
    }

    fn pairs(boxes: &[Self]) -> Vec<(u64, &Self, &Self)> {
        let n = boxes.len();
        let mut junction_pairs = Vec::with_capacity(n * (n - 1) / 2);

        for i in 0..boxes.len() {
            for j in i + 1..boxes.len() {
                let j1 = &boxes[i];
                let j2 = &boxes[j];
                let dist = j1.distance(&j2);
                junction_pairs.push((dist, j1, j2));
            }
        }

        junction_pairs.sort_unstable();
        junction_pairs
    }
}

pub fn solve() {
    let data = std::fs::read_to_string("input.txt")
        .expect("File should be present in the root directory.");

    let junction_boxes = data.lines().map(JunctionBox::new).collect::<Vec<_>>();
    let junction_pairs = JunctionBox::pairs(&junction_boxes);
    let mut circuits = Vec::with_capacity(junction_boxes.len());

    for (_, a, b) in junction_pairs {
        match (a.parent_circuit(&circuits), b.parent_circuit(&circuits)) {
            (Some(ca_idx), Some(cb_idx)) => {
                if ca_idx != cb_idx {
                    let other_circuit = std::mem::take(&mut circuits[cb_idx]);
                    circuits[ca_idx].extend(other_circuit);
                    circuits.swap_remove(cb_idx);
                }
            }
            (Some(ca_idx), None) => {
                circuits[ca_idx].insert(b);
            }
            (None, Some(cb_idx)) => {
                circuits[cb_idx].insert(a);
            }
            (None, None) => {
                circuits.push(HashSet::from([a, b]));
            }
        }

        if circuits.len() == 1 && circuits[0].len() == junction_boxes.len() {
            println!("Result: {}", a.x * b.x);
            break;
        }
    }

    // let mut lens = circuits.iter().map(|c| c.len()).collect::<Vec<_>>();
    // lens.sort();
    // lens.reverse();
    // println!("Final Res: {}", lens[0] * lens[1] * lens[2]);
}
