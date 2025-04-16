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
