### 1. Check formatting & code quality
``` bash
# Check formatting
cargo fmt --all

# Run clippy for code quality
cargo clippy --all
```

### 2. Build the Project
```bash
# Build all crates
cargo build --all

# Build with optimizations
cargo build --all --release
```

### 3. Run Tests
```bash
# Run all tests
cargo test --all

# Run tests with output
cargo test --all -- --nocapture

# Run specific crate tests
cargo test -p lumi_lexer
cargo test -p lumi_parser
cargo test -p lumi_vm
```

### 4. Run Benchmarks
```bash
# Run all benchmarks
cargo bench --all

# Run specific benchmarks
cargo bench -p lumi_lexer
```

## Quick Examples

### 1. Basic Lexical Analysis
```rust
use lumi_lexer::tokenize;

fn main() {
    let source = "let x = 42;";
    match tokenize(source) {
        Ok(tokens) => {
            println!("Found {} tokens:", tokens.len());
            for token in tokens {
                println!("  {:?}", token);
            }
        }
        Err(error) => {
            eprintln!("Lexical error: {}", error);
        }
    }
}