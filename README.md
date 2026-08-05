# Important
This project has been discontinued for the near future, the code debt accumulated from the poor decisions of my younger self, a highschooler, is just too big to handle now when adding more complex systems (like OOP), it'd require a near complete rewrite, which, at this point, might as well make a whole new project, which is what i will be doing. I learned a lot from this project and i will keep it up on my github as it is still my most impressive project so far and a proof of how far i've come for future me. 


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
- Builtin functions : `println`, `len`, `panic`
- F-string parsing : 
```scorie 
let x = 1;
let fstring = f"this is {x} fstring";  
```  

**Syntax:**
- Python like simplicity  
- Curly braces for blocks
- Semicolons as line terminators  
- Strong typing with type annotations on function parameters only

## Example
```scorie
fn add(x: Int, y: Int) {
    return x + y;
}

fn main() {
    let x = add(5, 2);
    let result = if x > 5 {
        value 10;
    } else {
        value 0;
    }
}

```

Compiles to Rust, then the user must use `rustc` to create an executable.

## What's Next

Likely backend architecture and optimizations. Language features will expand based on what's fun to build

## Installation

Requires Rust toolchain installed.
```bash
cargo install --path ./scorie-cli
```

## Usage
```bash
cd <current directory>  
scorie compile myfile.scorie output.rs  # Creates output.rs (also works without specifying file extensions)
rustc output.rs                     # Creates executable
```

## License

MIT
