# Fruitdata Quick Reference Guide

A fast lookup for common patterns and concepts used in the fruitdata project.

## Table of Contents
1. [Structs and Data](#structs-and-data)
2. [Enums and Pattern Matching](#enums-and-pattern-matching)
3. [Collections (Vec)](#collections-vec)
4. [File I/O](#file-io)
5. [Error Handling](#error-handling)
6. [CLI with Clap](#cli-with-clap)
7. [String Operations](#string-operations)
8. [References vs Ownership](#references-vs-ownership)
9. [Common Methods](#common-methods)

---

## Structs and Data

### Define a Struct
```rust
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FruitDimensions {
    pub name: String,
    pub length: f32,
    pub width: f32,
    pub height: f32,
}
```

**Key derives:**
- `Serialize` - convert to JSON
- `Deserialize` - convert from JSON
- `Debug` - print with `{:?}`
- `Clone` - create copies

### Create an Instance
```rust
let apple = FruitDimensions {
    name: "Apple".to_string(),
    length: 4.0,
    width: 2.5,
    height: 1.5,
};
```

### Add Methods to a Struct
```rust
impl FruitDimensions {
    pub fn volume(&self) -> f32 {
        self.length * self.width * self.height
    }
}

let vol = apple.volume();  // Call the method
```

---

## Enums and Pattern Matching

### Define an Enum
```rust
#[derive(Subcommand)]
enum Commands {
    List,
    Get { name: String },
    Add { name: String, length: f32, width: f32, height: f32 },
    Remove { name: String },
}
```

### Match on Enum Variants
```rust
match &cli.command {
    Commands::List => {
        println!("Listing...");
    }
    Commands::Get { name } => {
        println!("Getting: {}", name);
    }
    Commands::Add { name, length, width, height } => {
        println!("Adding: {}", name);
    }
    Commands::Remove { name } => {
        println!("Removing: {}", name);
    }
}
```

### Match with `if let` (Single Pattern)
```rust
// Shorter than full match when you only care about one case
if let Some(fruit) = fruits.iter().find(|f| f.name == "Apple") {
    println!("Found: {}", fruit.name);
}
```

### Match on Result
```rust
match load_catalogue(&file_path) {
    Ok(fruits) => {
        println!("Loaded {} fruits", fruits.len());
    }
    Err(e) => {
        eprintln!("Error: {}", e);
    }
}
```

---

## Collections (Vec)

### Create a Vector
```rust
let fruits: Vec<FruitDimensions> = vec![
    FruitDimensions { /* ... */ },
    FruitDimensions { /* ... */ },
];

let empty: Vec<FruitDimensions> = Vec::new();
```

### Add to Vector
```rust
let mut fruits = vec![];
fruits.push(apple);
fruits.push(banana);
```

### Iterate Over Vector
```rust
// Borrow each item (don't take ownership)
for fruit in &fruits {
    println!("{}", fruit.name);
}

// Take ownership (can't use fruits after this)
for fruit in fruits {
    println!("{}", fruit.name);
}

// With index
for (index, fruit) in fruits.iter().enumerate() {
    println!("{}: {}", index, fruit.name);
}
```

### Find Item
```rust
// Find first matching item
if let Some(fruit) = fruits.iter().find(|f| f.name == "Apple") {
    println!("Found: {}", fruit.name);
}

// Check if any item matches
if fruits.iter().any(|f| f.name.eq_ignore_ascii_case("apple")) {
    println!("Found apple");
}
```

### Filter/Remove Items
```rust
// Keep only items matching a condition
fruits.retain(|f| !f.name.eq_ignore_ascii_case("Apple"));
// After this, all "Apple" entries are gone

// Create new filtered vec
let large_fruits: Vec<_> = fruits
    .iter()
    .filter(|f| f.volume() > 20.0)
    .cloned()
    .collect();
```

### Get Length and Check if Empty
```rust
let len = fruits.len();
if fruits.is_empty() {
    println!("No fruits!");
}
```

---

## File I/O

### Read File to String
```rust
use std::fs;

let contents = fs::read_to_string("fruits.json")?;
println!("{}", contents);
```

### Write String to File
```rust
use std::fs;

fs::write("fruits.json", "file contents")?;
// Creates file if doesn't exist; overwrites if it does
```

### Parse JSON to Struct
```rust
use serde_json;

let json = r#"[{"name":"Apple","length":4.0,"width":2.5,"height":1.5}]"#;
let fruits: Vec<FruitDimensions> = serde_json::from_str(json)?;
```

### Convert Struct to JSON
```rust
use serde_json;

let apple = FruitDimensions { /* ... */ };
let json = serde_json::to_string_pretty(&apple)?;
println!("{}", json);
```

### Complete Load/Save Example
```rust
pub fn load_catalogue(path: &str) -> Result<Vec<FruitDimensions>, Box<dyn Error>> {
    let json = fs::read_to_string(path)?;
    let fruits = serde_json::from_str(&json)?;
    Ok(fruits)
}

pub fn save_catalogue(fruits: &[FruitDimensions], path: &str) -> Result<(), Box<dyn Error>> {
    let json = serde_json::to_string_pretty(fruits)?;
    fs::write(path, json)?;
    Ok(())
}
```

---

## Error Handling

### Using `?` Operator (Recommended)
```rust
fn main() -> Result<(), Box<dyn Error>> {
    let fruits = load_catalogue("fruits.json")?;  // Return error if this fails
    let json = serde_json::to_string(&fruits)?;  // Return error if this fails
    println!("{}", json);
    Ok(())  // Success!
}
```

The `?` operator:
- If operation succeeds: unwrap the `Ok` value and continue
- If operation fails: return the `Err` immediately from the function

### Using `match` (More Control)
```rust
match load_catalogue(&file_path) {
    Ok(fruits) => {
        println!("Loaded {} fruits", fruits.len());
    }
    Err(e) => {
        eprintln!("Failed to load: {}", e);
        return Err(e);
    }
}
```

### Using `.ok_or_else()` (Convert Option to Result)
```rust
let file_path = cli
    .file
    .to_str()
    .ok_or_else(|| "invalid file path".to_string())?;
```

Converts:
- `Some(value)` → `Ok(value)`
- `None` → `Err("invalid file path")`

### Common Error Patterns
```rust
// Pattern 1: Use ? to propagate error
let num: i32 = "5".parse()?;

// Pattern 2: Use match for custom handling
match "not_a_number".parse::<i32>() {
    Ok(n) => println!("{}", n),
    Err(e) => eprintln!("Parse error: {}", e),
}

// Pattern 3: Use unwrap() only for testing/panicking
let num = "5".parse::<i32>().unwrap();

// Pattern 4: Use expect() with custom message
let num = "5".parse::<i32>().expect("Must be a valid number");
```

---

## CLI with Clap

### Define CLI Structure
```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about)]
struct Cli {
    #[arg(short, long, default_value = "fruits.json")]
    file: PathBuf,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    List,
    Get { name: String },
    Add { name: String, length: f32, width: f32, height: f32 },
    Remove { name: String },
}
```

### Parse Arguments
```rust
fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();  // Automatically parses std::env::args()
    
    // cli.file is a PathBuf
    // cli.command is a Commands enum variant
    
    Ok(())
}
```

### Common Clap Attributes
```rust
#[arg(short)]              // Enable -f shorthand
#[arg(long)]               // Enable --file long form
#[arg(short, long)]        // Enable both -f and --file
#[arg(default_value = "")]  // Default if not provided
#[arg(required = true)]    // Must be provided
#[arg(value_name = "FILE")] // Name shown in help

#[command(subcommand)]     // This field contains subcommands
```

---

## String Operations

### String vs &str
```rust
// String: owned, can be modified, lives in heap
let name: String = "Apple".to_string();
let name: String = String::from("Apple");

// &str: borrowed reference, immutable, lives in data segment
let name: &str = "Apple";
```

### String Methods
```rust
let s = "  hello  ";

// Trimming whitespace
let trimmed = s.trim();        // "hello"
let trimmed = s.trim_start();  // "hello  "
let trimmed = s.trim_end();    // "  hello"

// Checking contents
if s.is_empty() { /* ... */ }
if s.contains("ell") { /* ... */ }
if s.starts_with("hel") { /* ... */ }
if s.ends_with("lo") { /* ... */ }

// Case operations
let lower = s.to_lowercase();
let upper = s.to_uppercase();

// Case-insensitive comparison
if s.eq_ignore_ascii_case("HELLO") { /* ... */ }

// Length
let len = s.len();  // Number of bytes
let chars = s.chars().count();  // Number of characters
```

### String Concatenation
```rust
let s1 = "Hello";
let s2 = "World";

// Method 1: format! macro
let result = format!("{} {}", s1, s2);

// Method 2: String concatenation
let mut result = String::from(s1);
result.push(' ');
result.push_str(s2);

// Method 3: join
let parts = vec![s1, s2];
let result = parts.join(" ");
```

---

## References vs Ownership

### Ownership (Take Responsibility)
```rust
let fruits = vec![/* ... */];
print_fruits(fruits);  // Ownership transferred to function
// fruits is no longer available here; ownership was moved
```

### Borrowing with References (Temporary Use)
```rust
let fruits = vec![/* ... */];
print_fruits(&fruits);  // Borrow (lend) to function
// fruits is still available here
```

### Mutable Borrowing (Borrow and Modify)
```rust
let mut fruits = vec![/* ... */];
add_fruit(&mut fruits, new_fruit);  // Borrow mutably
// fruits is still available, and has been modified
```

### References in Functions
```rust
// Take ownership (function can keep/modify/destroy)
fn process(fruits: Vec<FruitDimensions>) { /* ... */ }

// Borrow immutably (function can read, not modify)
fn display(fruits: &Vec<FruitDimensions>) { /* ... */ }
fn display(fruits: &[FruitDimensions]) { /* ... */ }  // More flexible

// Borrow mutably (function can modify)
fn modify(fruits: &mut Vec<FruitDimensions>) { /* ... */ }

// Return owned value (give to caller)
fn create() -> FruitDimensions { /* ... */ }
```

### The `*` Operator (Dereference)
```rust
let length: f32 = 4.0;
let length_ref: &f32 = &length;

// Dereference to access the value
let value = *length_ref;  // value is 4.0

// In function calls
let fruit = FruitDimensions {
    name: "Apple".to_string(),
    length: *length_ref,  // Dereference to get f32 from &f32
    // ...
};
```

---

## Common Methods

### Vec Methods
```rust
let mut v = vec![1, 2, 3];

v.push(4);              // Add to end
v.pop();                // Remove from end
v.insert(0, 0);         // Insert at index
v.remove(0);            // Remove at index
v.clear();              // Remove all items
v.len();                // Number of items
v.is_empty();           // Check if empty
v.contains(&2);         // Check if contains item
v.iter();               // Iterate over references
v.iter_mut();           // Iterate over mutable references
v.into_iter();          // Iterate, consuming vec
v.find();               // Find first matching
v.filter();             // Filter matching items
v.map();                // Transform each item
v.sort();               // Sort items
v.reverse();            // Reverse order
v.retain(|x| condition); // Keep items matching condition
v.clone();              // Create a copy
```

### String Methods
```rust
let s = "hello";

s.len();                 // Length in bytes
s.chars().count();       // Number of characters
s.to_uppercase();        // Convert to uppercase
s.to_lowercase();        // Convert to lowercase
s.trim();                // Remove whitespace
s.starts_with("hel");    // Check prefix
s.ends_with("lo");       // Check suffix
s.contains("ll");        // Check if contains substring
s.replace("l", "L");     // Replace substring
s.split(',');            // Split into parts
s.is_empty();            // Check if empty
```

### Option Methods
```rust
let opt: Option<i32> = Some(5);

opt.is_some();           // Check if Some
opt.is_none();           // Check if None
opt.unwrap();            // Get value or panic
opt.unwrap_or(0);        // Get value or default
opt.map(|x| x * 2);      // Transform the value
opt.filter(|x| x > &3);  // Filter based on condition
```

### Result Methods
```rust
let res: Result<i32, String> = Ok(5);

res.is_ok();             // Check if Ok
res.is_err();            // Check if Err
res.unwrap();            // Get value or panic
res.unwrap_or(0);        // Get value or default
res.map(|x| x * 2);      // Transform the value
res.map_err(|e| format!("Error: {}", e));  // Transform error
res.and_then(|x| Ok(x * 2));  // Chain Result-returning functions
```

---

## Pattern Matching Quick Reference

### Match Arms
```rust
match value {
    pattern1 => { /* execute if matches */ },
    pattern2 => { /* execute if matches */ },
    _ => { /* default case if nothing matches */ },
}
```

### Common Patterns
```rust
// Literal patterns
match age {
    0 => println!("Baby"),
    1..=12 => println!("Child"),
    13..=19 => println!("Teen"),
    _ => println!("Adult"),
}

// Enum patterns with extraction
match command {
    Commands::Add { name, value } => {
        println!("Adding {} with {}", name, value);
    }
    Commands::Remove { name } => {
        println!("Removing {}", name);
    }
    _ => println!("Other command"),
}

// Option patterns
match maybe_fruit {
    Some(fruit) => println!("Found: {}", fruit.name),
    None => println!("Not found"),
}

// Result patterns
match parse_result {
    Ok(number) => println!("Parsed: {}", number),
    Err(e) => println!("Error: {}", e),
}

// Multiple conditions (guards)
match fruit.volume() {
    v if v < 10.0 => println!("Small"),
    v if v < 50.0 => println!("Medium"),
    _ => println!("Large"),
}
```

---

## Debugging Tips

### Print Debug Info
```rust
println!("{:?}", fruit);     // Print struct with Debug trait
println!("{:#?}", fruit);    // Pretty-print struct
eprintln!("Error: {}", e);   // Print to stderr instead of stdout
```

### Use dbg! Macro
```rust
let result = dbg!(some_function());  // Prints value and location
let x = dbg!(5 + 6);  // Prints "20" and returns 20
```

### Check Compilation
```bash
cargo check      # Quick check for errors
cargo build      # Build binary
cargo test       # Run tests
cargo clippy     # Lint suggestions
```

---

## Type Conversions

### String Conversions
```rust
// To String
let s = "hello".to_string();
let s = String::from("hello");

// To &str
let s: &str = "hello";

// Parse to number
let num: i32 = "42".parse()?;
let num: f32 = "3.14".parse()?;

// Number to String
let s = (42).to_string();
let s = format!("{}", 42);
```

### PathBuf Conversions
```rust
let path = PathBuf::from("fruits.json");
let path_str: &str = path.to_str().ok_or("invalid path")?;
```

---