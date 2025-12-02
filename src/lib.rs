// The `lib.rs` serves as the entry point for the library module of this project.
// Here, we have chosen to organize the project into a hierarchical structure
// where all the "year2024" modules are encapsulated within the `year2024` folder.
//
// This approach keeps the project modular and maintainable:
// - Each year2024's logic resides in its dedicated file (e.g., `day1.rs`, `day2.rs`).
// - A central `mod.rs` inside the `year2024` folder declares and organizes all these modules.
// - This structure ensures that `lib.rs` remains concise and simply exposes the top-level `year2024` module.
//
// The `pub mod year2024;` line below exposes the entire `year2024` module to users of this library.

pub mod year2023;
pub mod year2024;
pub mod year2025;

pub fn read_test() -> String {
    std::fs::read_to_string("test.txt")
        .expect("input.txt must be present in the root of the directory.")
}

pub fn read_input() -> String {
    std::fs::read_to_string("input.txt")
        .expect("input.txt must be present in the root of the directory.")
}

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
