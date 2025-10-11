## Features

- List all fruits in the catalogue
- Show details (dimensions and computed volume) for a fruit by name
- Add a new fruit with length, width and height
- Remove a fruit by name
- Uses a JSON file as the single source of truth (default: `fruits.json`)

## Requirements

- Rust (1.50+ recommended; built with stable toolchain)
- Cargo (the Rust package manager)


## Quick start

Clone the repo and either run directly with Cargo or install the binary.

Run directly (development):
```bash
cargo run -- list
```

Build a release binary and run it:
```bash
cargo build --release
./target/release/fruitdata list
```

Install the binary to your Cargo bin directory (`~/.cargo/bin`) so you can run it from anywhere:
```bash
cargo install --path .
```

After installing, use:
```bash
fruitdata list
fruitdata get Mango
fruitdata add "Dragonfruit" 10.0 8.0 6.0
fruitdata remove Dragonfruit
```

All commands accept the `--file` / `-f` option to point to a specific JSON file:
```bash
fruitdata --file /path/to/myfruits.json list
```


## CLI usage examples

- List fruits:
```bash
fruitdata list
```

- Get details for a fruit (case-insensitive name matching):
```bash
fruitdata get Apple
```

- Add a fruit (name, length, width, height):
```bash
fruitdata add "Dragonfruit" 10.0 8.0 6.0
```
Note: Name must be non-empty and dimensions must be positive numbers. Duplicate names (case-insensitive) are rejected.

- Remove a fruit:
```bash
fruitdata remove Dragonfruit
```


## Data file format

The catalogue is a JSON array of objects with the following fields:

```json
[
  {
    "name": "Apple",
    "length": 4.0,
    "width": 2.5,
    "height": 1.5
  },
]
```

By default the CLI looks for `fruits.json` in the current working directory. Use `--file` to specify a custom path.
