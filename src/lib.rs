//! fruitdata - library crate
//!
//! This crate exposes the core functionality of the `fruitdata` project so it
//! can be used as a dependency by other Rust projects (local path, git or crates.io).
//!
//! Design notes:
//! - Keep IO and domain logic in `catalog` and `models` modules.
//! - Provide a thin library facade that re-exports the commonly used types and functions.
//!
//! Example
//! ```no_run
//! use fruitdata::{load_catalogue, FruitDimensions};
//!
//! fn print_names() -> Result<(), Box<dyn std::error::Error>> {
//!     let fruits = load_catalogue("fruits.json")?;
//!     for f in fruits {
//!         println!("{}", f.name);
//!     }
//!     Ok(())
//! }
//! ```

pub mod catalog;
pub mod models;

/// Re-export commonly used functions for consumers.
///
/// - `initialise_fruit_catalogue` creates the default catalogue.
/// - `load_catalogue` reads a catalogue from a JSON file.
/// - `save_catalogue` writes a catalogue to a JSON file.
pub use catalog::{initialise_fruit_catalogue, load_catalogue, save_catalogue};

/// The main data type representing a fruit and its dimensions.
pub use models::FruitDimensions;
