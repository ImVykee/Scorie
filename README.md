# Scorie

A higher-level programming language that compiles to Rust.

## Status

⚠️ **Experimental / Work in Progress**

Scorie is a learning project targeting developers ready to move beyond Python/JavaScript but not yet comfortable with Rust or C++.
The project is in its very early phases, it does not include any standard library or built-in functions, objects or methods, what is included in the `Features` category is the exhaustive list of working features

## Features

**Currently working:**
- Variables and functions
- Basic types: `Int`, `Float`, `String`, `Bool`
- Arithmetic and boolean operations
- If/else statements
- Explicit `return` (functions) and `value` (blocks) keywords

**Syntax:**
- Python-like: no semicolons, newlines as statement terminators
- Curly braces for blocks
- Strong typing with type annotations on function parameters only

## Example
```scorie
fn add(x: Int, y: Int) {
    return x + y
}

fn main() {
    let x = add(5, 2)
    let result = if x > 5 {
        value 10
    } else {
        value 0
    }
}

```

Compiles to Rust, then the user must use `rustc` to create an executable.

## What's Next

Currently working on a web playground. Language features will expand based on what's fun to build.

## Installation

Requires Rust toolchain installed.
```bash
cargo install --path .
```

## Usage
```bash
scorie compile myfile.scorie output.rs  # Creates output.rs (also works without specifying file extensions)
rustc output.rs                     # Creates executable
```

## License

MIT
