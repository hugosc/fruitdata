# fruitdata

A small, well-commented Rust CLI to manage a fruit catalogue (name and dimensions) persisted as JSON. This is a learning resource with intentional documentation and clear module organization.

## Quick Start

```bash
cargo run -- list
cargo run -- get Apple
cargo run -- add "Dragonfruit" 10.0 8.0 6.0
cargo run -- remove Dragonfruit
```

Use `--file` / `-f` to specify a custom JSON file.

## Features

- List all fruits
- Show details (dimensions and computed volume) for a fruit
- Add a fruit with length, width, and height
- Remove a fruit by name
- JSON persistence (default: `fruits.json`)

## Documentation

- **[LEARNING_GUIDE.md](LEARNING_GUIDE.md)** — In-depth walkthrough of the project and Rust concepts
- **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** — Copy-paste code snippets and common patterns

## Project Structure

- `src/main.rs` — CLI entry point and command dispatch (clap)
- `src/catalog.rs` — File I/O and JSON persistence (serde/serde_json)
- `src/models.rs` — Data structures and helpers

## Requirements

- Rust 1.50+ (stable toolchain)
- Cargo

## Installation

```bash
cargo install --path .
```

Then run `fruitdata` from anywhere.

## License

Educational resource—use and modify freely.