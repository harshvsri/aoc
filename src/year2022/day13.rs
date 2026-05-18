#[derive(Debug, PartialEq)]
pub enum Token {
    Open,
    Close,
    Num(u32),
}

pub struct Lexer<'a> {
    chars: std::iter::Peekable<std::str::Chars<'a>>,
    fake_tokens: Vec<Token>,
}

impl<'a> Lexer<'a> {
    pub fn new(s: &'a str) -> Self {
        Self {
            chars: s.chars().peekable(),
            fake_tokens: Vec::new(),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(t) = self.fake_tokens.pop() {
            return Some(t);
        }

        while let Some(&ch) = self.chars.peek() {
            match ch {
                '[' => {
                    self.chars.next();
                    return Some(Token::Open);
                }
                ']' => {
                    self.chars.next();
                    return Some(Token::Close);
                }
                c if c.is_ascii_digit() => {
                    let mut num = 0;
                    while let Some(&d) = self.chars.peek() {
                        if d.is_ascii_digit() {
                            num = (num * 10) + d.to_digit(10).unwrap();
                            self.chars.next();
                        } else {
                            break;
                        }
                    }
                    return Some(Token::Num(num));
                }
                ',' | _ => {
                    self.chars.next(); // Skip commas and Ignore unexpected chars
                }
            }
        }
        None
    }
}

pub fn eval(left_str: &str, right_str: &str) -> bool {
    let mut left = Lexer::new(left_str);
    let mut right = Lexer::new(right_str);

    loop {
        let l_tok = left.next();
        let r_tok = right.next();

        match (l_tok, r_tok) {
            (None, None) => return true,
            (None, Some(_)) => return true,  // Left ran out first
            (Some(_), None) => return false, // Right ran out first
            (Some(l), Some(r)) => {
                if l == r {
                    continue; // Both Open, both Close, or same Num
                }

                match (l, r) {
                    (Token::Num(nl), Token::Num(nr)) => return nl < nr,
                    (Token::Close, _) => return true, // Left list ended before right
                    (_, Token::Close) => return false, // Right list ended before left
                    (Token::Num(nl), Token::Open) => {
                        // Mixed type! Promote left to list
                        left.fake_tokens.push(Token::Close);
                        left.fake_tokens.push(Token::Num(nl));
                    }
                    (Token::Open, Token::Num(nr)) => {
                        // Mixed type! Promote right to list
                        right.fake_tokens.push(Token::Close);
                        right.fake_tokens.push(Token::Num(nr));
                    }
                    _ => unreachable!(),
                }
            }
        }
    }
}

pub fn solve() {
    let data =
        std::fs::read_to_string("test.txt").expect("File must be present in the root directory.");

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
        let in_order = eval(left, right);
        sum += (i + 1) * in_order as usize; // Clever hack to avoid branching
        println!("{} {} -> {}", left, right, in_order);
    }
    println!("Sum: {}", sum);
}
