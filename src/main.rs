mod catalog;
mod models;

use catalog::{initialise_fruit_catalogue, load_catalogue, save_catalogue};
use clap::{Parser, Subcommand};
use models::FruitDimensions;
use std::error::Error;
use std::path::PathBuf;

/// CLI arguments and subcommands for the fruitdata tool.
#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    /// Path to the fruit catalogue JSON file
    #[arg(short, long, default_value = "fruits.json")]
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all available fruits
    List,
    /// Show details for a fruit by name
    Get {
        /// Name of the fruit to display
        name: String,
    },
    /// Add a new fruit
    Add {
        /// Name of the fruit
        name: String,
        /// Length (positive number)
        length: f32,
        /// Width (positive number)
        width: f32,
        /// Height (positive number)
        height: f32,
    },
    /// Remove a fruit by name
    Remove {
        /// Name of the fruit to remove
        name: String,
    },
}

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

    // Convert file path to &str once (valid for the duration of main)
    let file_path = cli
        .file
        .to_str()
        .ok_or_else(|| "invalid file path".to_string())?
        .to_string();

    // Load or initialize the catalogue; make it mutable since we may change it
    let mut fruits = match load_catalogue(&file_path) {
        Ok(f) => f,
        Err(_) => {
            eprintln!("Could not load catalogue, initialising a new one.");
            initialise_fruit_catalogue()
        }
    };

    // Dispatch CLI subcommands
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
            fruits.retain(|f| !f.name.eq_ignore_ascii_case(name_trimmed));
            if fruits.len() < before {
                save_catalogue(&fruits, &file_path)?;
                println!("Removed '{}'.", name_trimmed);
            } else {
                println!("Fruit '{}' not found.", name_trimmed);
            }
        }
    }

    Ok(())
}
