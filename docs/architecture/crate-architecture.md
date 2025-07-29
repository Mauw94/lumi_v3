# Lumi_v3 Crate Architecture

## Workspace

```
lumi_v3/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── lumi_lexer/          # Lexical analysis             [X]
│   ├── lumi_ast/            # Abstract Syntax Tree         [X]
│   ├── lumi_parser/         # Syntax analysis              [-]
│   ├── lumi_semantic/       # Semantic analysis            [ ]
│   ├── lumi_bytecode/       # Bytecode generation          [ ]
│   ├── lumi_vm/             # Virtual Machine              [ ]
│   ├── lumi_runtime/        # Runtime environment          [ ]
│   ├── lumi_gc/             # Garbage collection           [ ]
│   └── lumi_api/            # Public API                   [ ]
└── tests/                 # Integration tests
```

## Individual Crate Architecture

### lumi_lexer

**Purpose** Converts source code into tokens

#### Architecture
```
src/
├── lib.rs          # Public API and module declarations
├── token.rs        # Token definitions and metadata
├── lexer.rs        # Main lexer implementation
└── error.rs        # Lexer-specific error types
```

#### Key Components
- **Token**: Represents individual lexical units with position information
- **TokenKind**: Enumeration of all possible token types
- **Lexer**: Main tokenization engine with state management
- **Position**: Source code position tracking (line/column)

#### Public API
```rust
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexError>
pub fn tokenize_fallback(source: &str) -> Vec<Token>
pub struct Lexer { /* ... */ }
pub struct Token { /* ... */ }
pub enum TokenKind { /* ... */ }
```

#### Dependencies
- `thiserror`: Error handling
- `serde`: Serialization support

### lumi_ast
**Purpose** Represents lumi program structure as Abstract Syntax Tree 

#### Architecture
```
src/
├── lib.rs          # Public API and module declarations
├── node.rs        # AST node definition
└── visitor.rs        # Visitor pattern implementation
```

#### Key Components
- **Node**: All AST node types
- **Visitor**: Visitor pattern implementation

#### Public API
```rust
pub enum Node { /* All AST node variants */ }
pub trait Visitor { /* Visitor pattern interface */ }
```

#### Dependencies
- `serde`: Serialization support


### lumi_parser
