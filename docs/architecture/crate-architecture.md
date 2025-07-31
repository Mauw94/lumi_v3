# Lumi_v3 Crate Architecture

## Workspace

```
lumi_v3/
├── Cargo.toml              # Workspace configuration
├── crates/
│   ├── lumi_lexer/          # Lexical analysis             [X]
│   ├── lumi_ast/            # Abstract Syntax Tree         [X]
│   ├── lumi_parser/         # Syntax analysis              [x]
│   ├── lumi_semantic/       # Semantic analysis            [x]
│   ├── lumi_bytecode/       # Bytecode generation          [ ]
│   ├── lumi_vm/             # Virtual Machine              [ ]
│   ├── lumi_runtime/        # Runtime environment          [ ]
│   ├── lumi_gc/             # Garbage collection           [ ]
│   └── lumi_api/            # Public API                   [ ]
└── tests/                   # Integration tests
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
├── node.rs         # AST node definition
└── visitor.rs      # Visitor pattern implementation
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

**Purpose**: Converts tokens into Abstract Syntax Tree

#### Architecture
```
src/
├── lib.rs          # Public API and module declarations
├── parser.rs       # Main parser implementation
├── error.rs        # Parser-specific error types
└── recovery.rs     # Error recovery mechanisms
```

#### Key Components
- **Parser**: Main parsing engine with recursive descent implementation
- **ParseResult**: Result type for parsing operations
- **Error Recovery**: Mechanisms for handling syntax errors gracefully

#### Public API
```rust
pub fn parse(tokens: &[Token]) -> ParseResult<Node>
pub struct Parser { /* ... */ }
pub enum ParseError { /* ... */ }
```

#### Dependencies
- `lumi_lexer`: Token input
- `lumi_ast`: AST output
- `thiserror`: Error handling

### lumi_semantic

**Purpose**: Validates program semantics and performs static analysis

#### Architecture
```
src/
├── lib.rs          # Public API and module declarations
├── analyzer.rs     # Main semantic analyzer
├── scope.rs        # Scope management
├── types.rs        # Type system
└── errors.rs       # Semantic error types
```

#### Key Components
- **SemanticAnalyzer**: Main analysis engine
- **Scope**: Variable and function scope management
- **Type**: Type system implementation
- **Error Detection**: Semantic error reporting

#### Public API
```rust
pub fn analyze(ast: &Node) -> Result<(), Vec<SemanticError>>
pub struct SemanticAnalyzer { /* ... */ }
pub struct Scope { /* ... */ }
pub enum Type { /* ... */ }
```

#### Dependencies
- `lumi_ast`: AST input
- `lumi_parser`: Parser for providing the AST for testing
- `thiserror`: Error handling

### lumi_bytecode

**Purpose**: Generates bytecode from Abstract Syntax Tree

#### Architecture
```
src/
├── lib.rs          # Public API and module declarations
├── generator.rs    # Bytecode generation engine
├── instructions.rs # Instruction set definition
└── tests.rs        # Internal test utilities
```

#### Key Components
- **BytecodeGenerator**: Main generation engine
- **Instruction**: Enumeration of all bytecode instructions
- **ConstantPool**: Optimization for literal storage

#### Public API
```rust
pub fn generate(ast: &Node) -> Bytecode
pub struct BytecodeGenerator { /* ... */ }
pub enum Instruction { /* ... */ }
pub struct ConstantPool { /* ... */ }
```

#### Dependencies
- `lumi_ast`: AST input
- `thiserror`: Error handling