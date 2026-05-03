pub fn gcd(a: u64, b: u64) -> u64 {
    let mut x = a;
    let mut y = b;

    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

pub fn lcm(a: u64, b: u64) -> u64 {
    if a == 0 || b == 0 {
        return 0;
    }
    (a / gcd(a, b)) * b
}

pub fn permutation(p: &mut [char], index: usize) {
    fn swap(p: &mut [char], x: usize, y: usize) {
        let temp = p[x];
        p[x] = p[y];
        p[y] = temp;
    }

    if index == p.len() {
        println!("{:?}", p);
    }
    for i in index..p.len() {
        swap(p, i, index);
        permutation(p, index + 1);
        swap(p, i, index);
    }
}

pub fn permute(chars: &mut [char], start: usize, results: &mut std::collections::HashSet<String>) {
    if start == chars.len() {
        results.insert(chars.iter().collect());
        return;
    }

    for i in start..chars.len() {
        chars.swap(start, i);
        permute(chars, start + 1, results);
        chars.swap(start, i);
    }
}
