// ============================================================================
// main.rs - CLI Application Entry Point
// ============================================================================
// This is the main entry point for the fruitdata CLI application.
// It handles:
// 1. Parsing command-line arguments (using the `clap` crate)
// 2. Loading/initializing the fruit catalogue from a JSON file
// 3. Dispatching to the appropriate command handler (list, get, add, remove)
// 4. Persisting changes back to the JSON file
//
// Key concepts:
// - CLI parsing: Converting strings from the command line into structured data
// - Pattern matching: Using Rust's `match` to handle different commands
// - Error handling: Using `Result<T, E>` for functions that can fail
// - String matching: Case-insensitive fruit name lookups
// ============================================================================

// Use the crate's library API so this binary becomes a thin wrapper.
// This allows other projects to depend on `fruitdata` as a library.
use fruitdata::{initialise_fruit_catalogue, load_catalogue, save_catalogue, FruitDimensions};
use clap::{Parser, Subcommand};
use std::error::Error;
use std::path::PathBuf;

// ============================================================================
// CLI ARGUMENT PARSING USING CLAP (Command Line Argument Parser)
// ============================================================================
// The `clap` crate automatically parses command-line arguments and generates
// help text, validates input, and builds the data structures below.
//
// When you run: `fruitdata --file myfruits.json list`
// Clap parses this into a Cli struct with:
// - file: PathBuf("myfruits.json")
// - command: Commands::List

/// The top-level CLI structure that represents all possible command-line arguments.
///
/// This struct tells clap how to parse the command line. The attributes
/// (lines starting with #[...]) are annotations that customize parsing behavior.
///
/// # How clap works
/// When the program runs, clap:
/// 1. Looks at std::env::args() (the arguments passed to the program)
/// 2. Matches them against this struct's fields and attributes
/// 3. Calls Cli::parse() which returns a populated Cli struct
/// 4. If parsing fails, it prints an error or help message and exits
///
/// # Example command lines
/// - `fruitdata list` → file="fruits.json", command=List
/// - `fruitdata -f custom.json get Apple` → file="custom.json", command=Get{name="Apple"}
/// - `fruitdata add Mango 5.0 3.0 2.5` → command=Add{name="Mango", ...}
#[derive(Parser)]
#[command(author, version, about)] // Auto-generate author/version from Cargo.toml
struct Cli {
    /// Path to the fruit catalogue JSON file.
    /// - Short form: `-f`
    /// - Long form: `--file`
    /// - Default value: `"fruits.json"` if not provided
    ///
    /// Examples:
    /// - `fruitdata list` (uses default fruits.json)
    /// - `fruitdata -f /tmp/fruits.json list`
    /// - `fruitdata --file ~/myfruits.json get Apple`
    #[arg(short, long, default_value = "fruits.json")]
    file: PathBuf,

    /// The subcommand to execute (list, get, add, or remove)
    /// Subcommands are positional arguments that determine which action to perform
    #[command(subcommand)]
    command: Commands,
}

/// An enum representing all possible subcommands (actions) the user can request.
///
/// In Rust, an `enum` is a type that can have multiple variants (possibilities).
/// Each variant can have associated data. For example, `Get { name: String }`
/// means the `Get` variant carries a String containing the fruit name.
///
/// # Why use an enum here?
/// This structure ensures:
/// - Type safety: The compiler ensures a command variant exists before we use it
/// - Exhaustiveness: We must handle all possible commands in our match statement
/// - Clear semantics: The code explicitly shows what actions are possible
#[derive(Subcommand)]
enum Commands {
    /// List all available fruits in the catalogue.
    /// Command: `fruitdata list`
    /// Does not require any additional arguments.
    List,

    /// Show detailed information for a specific fruit.
    /// Command: `fruitdata get AppleName`
    ///
    /// The `name` field will be populated with the fruit name provided by the user.
    /// Example: `fruitdata get Apple` → Get { name: "Apple" }
    Get {
        /// The name of the fruit to look up
        name: String,
    },

    /// Add a new fruit to the catalogue.
    /// Command: `fruitdata add "FruitName" 4.0 2.5 1.5`
    ///
    /// All fields must be provided in order: name, length, width, height
    /// The name can contain spaces if quoted (e.g., "Dragon Fruit")
    Add {
        /// Name of the fruit (e.g., "Apple", "Dragonfruit")
        name: String,
        /// Length dimension (must be a positive number)
        length: f32,
        /// Width dimension (must be a positive number)
        width: f32,
        /// Height dimension (must be a positive number)
        height: f32,
    },

    /// Remove a fruit from the catalogue by name.
    /// Command: `fruitdata remove AppleName`
    ///
    /// After removal, the catalogue is saved back to the JSON file.
    Remove {
        /// The name of the fruit to remove
        name: String,
    },
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Display detailed information about a fruit in a human-readable format.
///
/// This helper function formats a FruitDimensions struct nicely for console output.
/// It's used by the `Get` command to show fruit details to the user.
///
/// # Arguments
/// - `fruit: &FruitDimensions` - A reference to the fruit to display
///   (We use & to borrow the data without taking ownership)
///
/// # Output format
/// ```
/// Name: Apple
/// Dimensions: 4.0 x 2.5 x 1.5
/// Volume: 15.0
/// ```
///
/// # Example
/// ```
/// let apple = FruitDimensions {
///     name: "Apple".to_string(),
///     length: 4.0,
///     width: 2.5,
///     height: 1.5,
/// };
/// display_fruit_info(&apple);
/// ```
fn display_fruit_info(fruit: &FruitDimensions) {
    println!("Name: {}", fruit.name);
    println!(
        "Dimensions: {} x {} x {}",
        fruit.length, fruit.width, fruit.height
    );
    println!("Volume: {}", fruit.volume());
}

// ============================================================================
// MAIN FUNCTION - Program Entry Point
// ============================================================================

/// The main function is the entry point where the program starts execution.
///
/// # Why does main() return Result?
/// `Result<(), Box<dyn Error>>` means:
/// - `Ok(())` - Program executed successfully (no data to return, just success)
/// - `Err(...)` - An error occurred; the error is boxed (stored on the heap)
///
/// Returning Result from main() is a Rust best practice because:
/// 1. It allows us to use the `?` operator for error propagation
/// 2. If main() returns an error, Rust automatically exits with status code 1
/// 3. It makes error handling cleaner and less verbose
///
/// # Program flow
/// 1. Parse CLI arguments into a Cli struct
/// 2. Convert the file path (PathBuf) to a string
/// 3. Load catalogue from JSON (or initialize a new one if file doesn't exist)
/// 4. Match on the command and execute the appropriate action
/// 5. Return Ok(()) on success or propagate errors with ?
fn main() -> Result<(), Box<dyn Error>> {
    // ========================================================================
    // STEP 1: Parse command-line arguments
    // ========================================================================
    // Cli::parse() reads std::env::args() and constructs a Cli struct.
    // If parsing fails (e.g., invalid arguments), clap prints an error and exits.
    // If parsing succeeds, we have a fully populated Cli struct.
    let cli = Cli::parse();

    // ========================================================================
    // STEP 2: Convert PathBuf to &str
    // ========================================================================
    // `cli.file` is a PathBuf (an owned path). We need to convert it to &str
    // (a string reference) to pass to our catalogue functions.
    //
    // Why this is complex:
    // - PathBuf might contain invalid UTF-8 characters (rare but possible)
    // - .to_str() returns Option<&str>, which is Some(s) if valid, None if invalid
    // - We use .ok_or_else() to convert None into an error
    // - Then .to_string() converts &str to String for storage
    let file_path = cli
        .file
        .to_str()
        .ok_or_else(|| "invalid file path".to_string())?
        .to_string();

    // ========================================================================
    // STEP 3: Load or initialize the catalogue
    // ========================================================================
    // Try to load the catalogue from the JSON file.
    // If loading fails (file doesn't exist, corrupted JSON, etc.),
    // fall back to initializing a new default catalogue.
    //
    // After this point, `fruits` contains a list of FruitDimensions structs,
    // either loaded from disk or freshly initialized.
    //
    // We use `mut` (mutable) because some commands (Add, Remove) will modify it.
    let mut fruits = match load_catalogue(&file_path) {
        Ok(f) => {
            // Successfully loaded catalogue from file
            f
        }
        Err(_) => {
            // File doesn't exist or is corrupted; create a default catalogue
            eprintln!("Could not load catalogue, initialising a new one.");
            initialise_fruit_catalogue()
        }
    };

    // ========================================================================
    // STEP 4: Dispatch to the appropriate command handler
    // ========================================================================
    // We use Rust's `match` statement to handle each possible command.
    // The match statement is exhaustive - we must handle all enum variants.
    // This is part of Rust's safety: the compiler ensures we don't forget a case.
    //
    // We match on `&cli.command` (a reference) so we don't move/consume the data.
    match &cli.command {
        // ====================================================================
        // COMMAND: list
        // ====================================================================
        // List all fruits in the catalogue (just their names)
        Commands::List => {
            println!("--- Available Fruits ---");
            // Iterate over all fruits; `&fruits` gives us references to each
            for f in &fruits {
                println!("{}", f.name);
            }
        }

        // ====================================================================
        // COMMAND: get <name>
        // ====================================================================
        // Find and display details for a specific fruit by name
        Commands::Get { name } => {
            // Use `iter().find()` to locate the first fruit matching the name.
            // .find() takes a closure (a small anonymous function) and returns
            // an Option: Some(fruit) if found, None if not found.
            //
            // `eq_ignore_ascii_case()` compares names case-insensitively:
            // "apple", "Apple", "APPLE" all match.
            if let Some(fruit) = fruits.iter().find(|f| f.name.eq_ignore_ascii_case(name)) {
                // Found a matching fruit; display its details
                display_fruit_info(fruit);
            } else {
                // No matching fruit found; inform the user
                println!("Fruit '{}' not found.", name);
            }
        }

        // ====================================================================
        // COMMAND: add <name> <length> <width> <height>
        // ====================================================================
        // Add a new fruit to the catalogue with the given dimensions
        Commands::Add {
            name,
            length,
            width,
            height,
        } => {
            // Validation 1: Ensure the name is not empty (after trimming whitespace)
            let name_trimmed = name.trim();
            if name_trimmed.is_empty() {
                println!("Name must not be empty.");
                return Ok(()); // Exit the command; don't add anything
            }

            // Validation 2: Ensure all dimensions are positive numbers
            // f32 can be zero or negative, which doesn't make physical sense
            if *length <= 0.0 || *width <= 0.0 || *height <= 0.0 {
                println!("Dimensions must be positive numbers.");
                return Ok(());
            }

            // Validation 3: Ensure the fruit doesn't already exist (case-insensitive)
            // We don't allow duplicate fruits in the catalogue
            if fruits
                .iter()
                .any(|f| f.name.eq_ignore_ascii_case(name_trimmed))
            {
                println!("Fruit '{}' already exists.", name_trimmed);
                return Ok(());
            }

            // All validations passed; create the new fruit struct
            let fruit = FruitDimensions {
                name: name_trimmed.to_string(),
                length: *length, // Dereference (convert &f32 to f32)
                width: *width,
                height: *height,
            };

            // Add the fruit to our in-memory catalogue
            fruits.push(fruit);

            // Persist the changes to the JSON file
            // If saving fails, the ? operator will return the error
            save_catalogue(&fruits, &file_path)?;

            println!("Added '{}'.", name_trimmed);
        }

        // ====================================================================
        // COMMAND: remove <name>
        // ====================================================================
        // Remove a fruit from the catalogue by name (case-insensitive)
        Commands::Remove { name } => {
            // Validation: Ensure the name is not empty (after trimming)
            let name_trimmed = name.trim();
            if name_trimmed.is_empty() {
                println!("Name must not be empty.");
                return Ok(());
            }

            // Remember how many fruits we had before removal
            let before = fruits.len();

            // Remove all fruits matching the name (case-insensitive)
            // `.retain()` keeps only the fruits for which the closure returns true.
            // Here
