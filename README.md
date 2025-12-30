# Lumi V3

**Lumi** is a custom programming language with a Rust-based engine inspired by the design and performance characteristics of [Google’s V8 JavaScript engine](https://v8.dev/). Lumi is built from the ground up with modular, well-typed components to tokenize, parse, analyze, and eventually execute Lumi programs.

---

## Project Overview

Lumi is designed for:

* Custom syntax and semantics
* Static type checking
* Bytecode-based execution
* High performance and safety (thanks to Rust)

---

## Workspace Layout

```txt
lumi_v3/
├── crates/
│   ├── lumi_lexer/       # Lexical analysis
│   ├── lumi_ast/         # Abstract Syntax Tree
│   ├── lumi_parser/      # Syntax analysis
│   ├── lumi_semantic/    # Semantic analysis
│   ├── lumi_bytecode/    # Bytecode generation 
│   ├── lumi_vm/          # Virtual Machine 
│   ├── lumi_runtime/     # Runtime environment (TBD)
│   ├── lumi_gc/          # Garbage collection (TBD)
│   └── lumi_api/         # Public API (TBD)
├── tests/                # Integration tests (TBD)
├── docs/                 # Design docs & specifications
└── Cargo.toml            # Workspace configuration
```

---

## High-Level Architecture

``` text
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Source Code   │───▶│   Lexical       │───▶│   Syntax        │
│   lumi lang     │    │   Analysis      │    │   Analysis      │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │                       │
                                ▼                       ▼
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Bytecode      │◀───│   Semantic      │◀───│   Abstract      │
│   Execution     │    │   Analysis      │    │   Syntax Tree   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │   Runtime       │
                       │   Environment   │
                       └─────────────────┘
```

---

## Component Overview

### `lumi_lexer` - Lexical Analysis

* **Input**: Source code string
* **Output**: Stream of tokens
* **Features**:

  * Position tracking
  * Token recovery
  * Unicode-ready (WIP)

```rust
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError>
```

---

### `lumi_ast` - Abstract Syntax Tree

* **Input**: Token stream
* **Output**: Tree-based program structure
* **Features**:

  * Source location tracking
  * Visitor pattern support

```rust
pub enum Node { /* ... */ }
pub trait Visitor { /* ... */ }
```

---

### `lumi_parser` - Syntax Analysis

* **Input**: Tokens
* **Output**: AST
* **Features**:

  * Recursive descent parsing
  * Error recovery
  * ParseResult with error context

```rust
pub fn parse(tokens: &[Token]) -> ParseResult<Node>
```

---

### `lumi_semantic` - Semantic Analysis

* **Input**: AST
* **Output**: Type-checked AST
* **Features**:

  * Type checking
  * Scope validation
  * Semantic error handling

```rust
pub fn analyze(ast: &Node) -> Result<(), Vec<SemanticError>>
```

---

### `lumi_bytecode` - Bytecode Generation

- **Input**: AST
- **Output**: Lumi Bytecode
- **Features**:
  - Instruction generation
  - Constant pool optimization
  - Expandable for custom instruction sets
    - **Symbol table for variable management**
    - Maps variable names to indices for efficient storage and lookup
    - Ensures correct variable scoping and access during bytecode generation
    - Automatically assigns indices to new variables and reuses them for subsequent references


```rust
pub fn generate(ast: &Node) -> Bytecode
```

---

### `lumi_vm` - Virtual Machine

- **Input**: Lumi Bytecode
- **Output**: Runtime execution result
- **Features**:
  - Stack-based execution model
  - Register and call frame support
  - Instruction interpretation

```rust
pub fn execute(bytecode: &Bytecode) -> Result<Value, VMError>
```

### `lumi_runtime` - Runtime Environment

- **Purpose**: Runtime environment and value system
- **Features**:
  - Context handling
  - Object and function models
  - Value representation for dynamic types

## Dependencies

Common crates used:

* [`thiserror`](https://docs.rs/thiserror) - For ergonomic error handling
* [`serde`](https://docs.rs/serde) - For AST/token serialization

---

## Build & Test

```bash
# Check formatting
cargo fmt --all

# Build the entire workspace
cargo build --all

# Run all tests
cargo test --all
```

---

## Roadmap

* [x] Lexer                     
* [x] Parser
* [x] AST
* [x] Semantic Analyzer
* [x] Bytecode Generator
* [x] Virtual Machine
* [ ] Garbage Collector
* [x] Runtime
* [ ] Public API

🛠 Note: While the foundational crates (lumi_lexer, lumi_ast, lumi_parser, lumi_semantic, lumi_bytecode, lumi_vm and lumi_runtime) are implemented, they are actively under development. Functionality is expanding as Lumi evolves into a more expressive and capable language. Expect breaking changes, experimental features, and rapid iteration.
---

## License

This project is licensed under the MIT License. See [LICENSE](./LICENSE) for details.
