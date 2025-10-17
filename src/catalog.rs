// ============================================================================
// catalog.rs - File I/O and Data Persistence
// ============================================================================
// This module handles all interactions with the JSON file that stores our
// fruit catalogue. It provides three main functions:
//
// 1. load_catalogue() - Read fruits from a JSON file into memory
// 2. save_catalogue() - Write fruits from memory to a JSON file
// 3. initialise_fruit_catalogue() - Create a default catalogue if the file
//    doesn't exist or can't be read
//
// Key concept: Persistence means data survives when the program exits.
// Without these functions, changes to the fruit list would disappear when
// the CLI program terminates. By saving to JSON files, we preserve the data.
// ============================================================================

use crate::models::FruitDimensions;
use std::error::Error;
use std::fs;

/// Load the fruit catalogue from a JSON file.
///
/// This function reads a JSON file from the filesystem and parses it into
/// a Vec (vector/list) of FruitDimensions structs.
///
/// # How it works
/// 1. `fs::read_to_string(path)` reads the entire file into a String
/// 2. `serde_json::from_str(&json)` parses the JSON string into Rust structs
/// 3. If either step fails, we return the error wrapped in a Box
///
/// # Arguments
/// - `path: &str` - The filesystem path to the JSON file (e.g., "fruits.json")
///
/// # Returns
/// - `Ok(Vec<FruitDimensions>)` - Successfully loaded list of fruits
/// - `Err(Box<dyn Error>)` - An error occurred (file not found, invalid JSON, etc.)
///
/// # Error Cases
/// - File doesn't exist at the given path
/// - File can't be read (permission denied)
/// - JSON is malformed (invalid syntax)
/// - JSON structure doesn't match FruitDimensions (missing fields, wrong types)
///
/// # Example Usage
/// ```
/// match load_catalogue("fruits.json") {
///     Ok(fruits) => println!("Loaded {} fruits", fruits.len()),
///     Err(e) => eprintln!("Failed to load: {}", e),
/// }
/// ```
pub fn load_catalogue(path: &str) -> Result<Vec<FruitDimensions>, Box<dyn Error>> {
    // Step 1: Read the entire file contents into a String
    // The `?` operator means "if this fails, return the error immediately"
    let json = fs::read_to_string(path)?;

    // Step 2: Parse the JSON string into a Vec of FruitDimensions
    // serde_json automatically uses the #[derive(Deserialize)] we set up in models.rs
    // to know how to convert JSON into our struct
    let fruits = serde_json::from_str(&json)?;

    // Step 3: Return the successfully loaded fruits
    Ok(fruits)
}

/// Save the fruit catalogue to a JSON file.
///
/// This function converts a slice of FruitDimensions structs into pretty-printed
/// JSON and writes it to a file at the specified path. This is how we persist
/// changes made by the user (add/remove commands).
///
/// # How it works
/// 1. `serde_json::to_string_pretty(fruits)` converts our Rust structs to formatted JSON
/// 2. `fs::write(path, json)` writes the JSON string to the filesystem
/// 3. If either step fails, we return the error
///
/// # Arguments
/// - `fruits: &[FruitDimensions]` - A slice (reference to a list) of fruits to save
///   We use a slice (&[...]) instead of a Vec to be flexible about where the data comes from
/// - `path: &str` - The filesystem path where the JSON will be written
///
/// # Returns
/// - `Ok(())` - Successfully saved the catalogue (unit type `()` means no data returned)
/// - `Err(Box<dyn Error>)` - An error occurred (disk full, permission denied, etc.)
///
/// # Error Cases
/// - Path doesn't exist or is invalid
/// - No write permission for the file/directory
/// - Disk is full
/// - JSON serialization fails (shouldn't happen with valid FruitDimensions)
///
/// # Side Effects
/// - Creates the file if it doesn't exist
/// - Overwrites the file if it already exists
/// - Writes formatted/indented JSON (easier to read manually)
///
/// # Example Usage
/// ```
/// let fruits = vec![
///     FruitDimensions { name: "Apple".into(), length: 4.0, width: 2.5, height: 1.5 },
/// ];
/// if let Err(e) = save_catalogue(&fruits, "fruits.json") {
///     eprintln!("Failed to save: {}", e);
/// }
/// ```
pub fn save_catalogue(fruits: &[FruitDimensions], path: &str) -> Result<(), Box<dyn Error>> {
    // Step 1: Convert Rust structs to pretty-printed JSON string
    // `to_string_pretty` adds indentation and line breaks for readability
    // (as opposed to `to_string` which produces compact JSON)
    let json = serde_json::to_string_pretty(fruits)?;

    // Step 2: Write the JSON string to the filesystem
    // This creates the file if it doesn't exist, or overwrites it if it does
    fs::write(path, json)?;

    // Step 3: Return success (unit type `()` is Rust's way of saying "nothing to return")
    Ok(())
}

/// Create and return a default catalogue of fruits.
///
/// This function is called when the programme can't load an existing catalogue
/// (e.g., the first time the user runs fruitdata, or if the file is deleted).
/// It provides a sensible starting point with a few common fruits.
///
/// # Why this exists
/// Instead of requiring the user to manually create a JSON file, we provide
/// a default catalogue. This makes the user experience smoother.
///
/// # Returns
/// - `Vec<FruitDimensions>` - A vector containing the default fruits
///
/// # Fruits in the default catalogue
/// - Orange: 5.0 × 3.0 × 2.0
/// - Apple: 4.0 × 2.5 × 1.5
/// - Banana: 6.0 × 3.5 × 2.5
/// - Pear: 6.0 × 3.5 × 2.5
///
/// # Example Usage
/// ```
/// let fruits = initialise_fruit_catalogue();
/// println!("Default catalogue has {} fruits", fruits.len()); // prints: 4
/// ```
pub fn initialise_fruit_catalogue() -> Vec<FruitDimensions> {
    // Use `vec![]` macro to create a vector with initial values
    // Each FruitDimensions is constructed with specific dimensions
    vec![
        // Orange - Medium-sized, roughly spherical
        FruitDimensions {
            name: "Orange".into(), // .into() converts &str to String
            length: 5.0,
            width: 3.0,
            height: 2.0,
        },
        // Apple - Small, roughly spherical
        FruitDimensions {
            name: "Apple".into(),
            length: 4.0,
            width: 2.5,
            height: 1.5,
        },
        // Banana - Long and thin, elongated
        FruitDimensions {
            name: "Banana".into(),
            length: 6.0,
            width: 3.5,
            height: 2.5,
        },
        // Pear - Similar to banana, slightly different proportions
        FruitDimensions {
            name: "Pear".into(),
            length: 6.0,
            width: 3.5,
            height: 2.5,
        },
    ]
}
