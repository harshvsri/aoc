// The `lib.rs` serves as the entry point for the library module of this project.
// Here, we have chosen to organize the project into a hierarchical structure
// where all the "yearXXXX" modules are encapsulated within the `yearXXXX` folder.
//
// This approach keeps the project modular and maintainable:
// - Each year's logic resides in its dedicated file (e.g., `day1.rs`, `day2.rs`).
// - A central `mod.rs` inside the `yearXXXX` folder declares and organizes all these modules.
// - This structure ensures that `lib.rs` remains concise and simply exposes the top-level module.

pub mod macros;
pub mod utils;

pub mod year2022;
pub mod year2023; // 50*
pub mod year2024; // 50*
pub mod year2025; // 24*
