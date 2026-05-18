#[derive(Debug, Eq, PartialEq)]
pub enum Packet {
    Int(u32),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Int(l), Packet::Int(r)) => l.cmp(r),
            (Packet::List(l), Packet::List(r)) => l.cmp(r),
            (Packet::Int(l), Packet::List(r)) => vec![Packet::Int(*l)].cmp(r),
            (Packet::List(l), Packet::Int(r)) => l.cmp(&vec![Packet::Int(*r)]),
        }
    }
}

impl Packet {
    pub fn parse(s: &str) -> Self {
        let mut chars = s.chars().peekable();
        Self::parse_packet(&mut chars)
    }

    fn parse_packet(chars: &mut std::iter::Peekable<std::str::Chars>) -> Self {
        if let Some(&ch) = chars.peek() {
            match ch {
                '[' => {
                    chars.next();
                    let mut list = Vec::new();
                    if let Some(&']') = chars.peek() {
                        chars.next();
                        return Packet::List(list);
                    }

                    loop {
                        list.push(Self::parse_packet(chars));
                        if let Some(&ch) = chars.peek() {
                            match ch {
                                ',' => chars.next(),
                                ']' => {
                                    chars.next();
                                    break;
                                }
                                _ => panic!("unexpected char: {}", ch),
                            };
                        } else {
                            panic!("unexpected end of input");
                        }
                    }
                    Packet::List(list)
                }
                ch if ch.is_ascii_digit() => {
                    let mut num_str = String::new();
                    while let Some(&ch) = chars.peek() {
                        if ch.is_ascii_digit() {
                            num_str.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    Packet::Int(num_str.parse().unwrap())
                }
                _ => panic!("unexpected starting char: {}", ch),
            }
        } else {
            panic!("unexpected end of input");
        }
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("input.txt").expect("File must be present in the root directory.");

    let packets = data
        .trim()
        .split("\n\n")
        .map(|packet| {
            packet
                .split_once("\n")
                .expect("Must be separated by new line char.")
        })
        .collect::<Vec<_>>();

    let mut sum = 0;
    for (i, (left, right)) in packets.iter().enumerate() {
        let left_packet = Packet::parse(left);
        let right_packet = Packet::parse(right);

        let in_order = left_packet < right_packet;
        sum += (i + 1) * in_order as usize;
        // println!("{} {} -> {}", left, right, in_order);
    }
    println!("Sum: {}", sum);

    let mut packets = packets
        .iter()
        .flat_map(|(l, r)| [Packet::parse(l), Packet::parse(r)])
        .collect::<Vec<_>>();
    packets.push(Packet::parse("[[2]]"));
    packets.push(Packet::parse("[[6]]"));

    packets.sort();

    println!(
        "Res: {}",
        (packets.binary_search(&Packet::parse("[[2]]")).unwrap() + 1)
            * (packets.binary_search(&Packet::parse("[[6]]")).unwrap() + 1)
    );
}
