// ============================================================================
// main.rs - CLI Application Entry Point (binary wrapper)
// ============================================================================
// This binary is a thin wrapper around the `fruitdata` library. The core logic
// (models + catalog I/O) lives in `src/lib.rs` and its modules. Keeping the
// binary small makes the library reusable by other crates (path/git/crates.io).
// ============================================================================

use crate::{initialise_fruit_catalogue, load_catalogue, save_catalogue, FruitDimensions};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

/// Top-level CLI using clap
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the fruit catalogue JSON file.
    #[arg(short, long, default_value = "fruits.json")]
    file: PathBuf,

    /// Which command to run.
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all fruits in the catalogue.
    List,

    /// Show details for a specific fruit.
    Get {
        /// Fruit name to lookup.
        name: String,
    },

    /// Add a new fruit with dimensions.
    Add {
        name: String,
        length: f32,
        width: f32,
        height: f32,
    },

    /// Remove a fruit by name.
    Remove { name: String },
}

/// Pretty-print a fruit's details to stdout.
fn display_fruit_info(fruit: &FruitDimensions) {
    println!("Name: {}", fruit.name);
    println!(
        "Dimensions: {} x {} x {}",
        fruit.length, fruit.width, fruit.height
    );
    println!("Volume: {}", fruit.volume());
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let file_path = cli
        .file
        .to_str()
        .ok_or_else(|| "invalid file path".to_string())?
        .to_string();

    // Load catalogue or initialise default if loading fails.
    let mut fruits = match load_catalogue(&file_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Could not load catalogue, initialising a new one.");
            initialise_fruit_catalogue()
        }
    };

    match &cli.command {
        Commands::List => {
            println!("--- Available Fruits ---");
            for f in &fruits {
                println!("{}", f.name);
            }
        }

        Commands::Get { name } => {
            if let Some(fruit) = fruits.iter().find(|f| f.name.eq_ignore_ascii_case(name)) {
                display_fruit_info(fruit);
            } else {
                println!("Fruit '{}' not found.", name);
            }
        }

        Commands::Add {
            name,
            length,
            width,
            height,
        } => {
            let name_trimmed = name.trim();
            if name_trimmed.is_empty() {
                println!("Name must not be empty.");
                return Ok(());
            }

            if *length <= 0.0 || *width <= 0.0 || *height <= 0.0 {
                println!("Dimensions must be positive numbers.");
                return Ok(());
            }

            if fruits
                .iter()
                .any(|f| f.name.eq_ignore_ascii_case(name_trimmed))
            {
                println!("Fruit '{}' already exists.", name_trimmed);
                return Ok(());
            }

            let fruit = FruitDimensions {
                name: name_trimmed.to_string(),
                length: *length,
                width: *width,
                height: *height,
            };

            fruits.push(fruit);
            save_catalogue(&fruits, &file_path)?;
            println!("Added '{}'.", name_trimmed);
        }

        Commands::Remove { name } => {
            let name_trimmed = name.trim();
            if name_trimmed.is_empty() {
                println!("Name must not be empty.");
                return Ok(());
            }

            let before = fruits.len();
            // Remove matching fruits (case-insensitive)
            fruits.retain(|f| !f.name.eq_ignore_ascii_case(name_trimmed));
            let after = fruits.len();

            if after == before {
                println!("No fruit named '{}' found.", name_trimmed);
            } else {
                save_catalogue(&fruits, &file_path)?;
                println!("Removed '{}'.", name_trimmed);
            }
        }
    }

    Ok(())
}
